import { Archive, Brain, Clock3, Compass, Home, Library, Map as MapIcon, Search, User } from "lucide-react";
import React from "react";
import { useAtomValue, useSetAtom } from "jotai";
import { appShellStateAtom, patchAppShellStateAtom, type AppShellState, type ViewMode } from "../../app-shell-state";
import { useCatalogState, usePracticeIndex, useStoryBundle, useTextbookState } from "./player-hooks";
import type { CourseRecord, CourseSummary, PlayerProgress, StoryManifest, TermEntry } from "./player-model";
import { EmptyState, LoadingState } from "./player-primitives";
import { markStepCompleteAtom, playerProgressAtom, rateReviewAtom, rememberStepAtom } from "./player-state";
import { ChapterHomeView } from "./views/ChapterHomeView";
import { CourseHomeView } from "./views/CourseHomeView";
import { LearningDock } from "./views/LearningDock";
import { LibraryView } from "./views/LibraryView";
import { ReviewView } from "./views/ReviewView";
import { StoryOverlay } from "./views/StoryOverlay";
import { TermDrawer } from "./views/TermDrawer";
import { TextbookView } from "./views/TextbookView";
import { VaultView } from "./views/VaultView";
import { EMPTY_COURSES, EMPTY_MANIFESTS, EMPTY_STEPS, chapterKey, getCourseContinueTarget, isCourseReady } from "./player-utils";
import "./App.css";

function buildCourseStats(
  courses: CourseRecord[],
  manifests: Record<string, StoryManifest>,
  progress: PlayerProgress
) {
  return new Map<string, CourseSummary>(
    courses.map((course) => {
      const chapterStats = course.chapters.map((chapter) => {
        const manifest = manifests[chapterKey(course.courseId, chapter.chapterId)];
        const totalSteps = manifest?.steps?.length ?? 0;
        const completed = progress.completedSteps[chapterKey(course.courseId, chapter.chapterId)] ?? [];
        const completionRatio = totalSteps === 0 ? 0 : completed.length / totalSteps;
        return {
          chapterId: chapter.chapterId,
          totalSteps,
          completedSteps: completed.length,
          completionRatio
        };
      });

      const totalSteps = chapterStats.reduce((sum, item) => sum + item.totalSteps, 0);
      const completedSteps = chapterStats.reduce((sum, item) => sum + item.completedSteps, 0);
      const mastery = totalSteps === 0 ? 0 : Math.round((completedSteps / totalSteps) * 100);
      return [
        course.courseId,
        {
          totalSteps,
          completedSteps,
          mastery,
          chapters: chapterStats
        }
      ] as const;
    })
  );
}

function countDueCards(practice: ReturnType<typeof usePracticeIndex>, progress: PlayerProgress) {
  if (!practice.data) {
    return 0;
  }

  return practice.data.flashcards.filter((card) => {
    const questionId = card.variant.question_id;
    if (!questionId) {
      return true;
    }
    const history = progress.reviewHistory[questionId];
    return !history || history.lastRating < 4;
  }).length;
}

export function App() {
  const catalog = useCatalogState();
  const progress = useAtomValue(playerProgressAtom);
  const navigation = useAtomValue(appShellStateAtom);
  const patchNavigation = useSetAtom(patchAppShellStateAtom);
  const markStepComplete = useSetAtom(markStepCompleteAtom);
  const rememberStep = useSetAtom(rememberStepAtom);
  const rateReview = useSetAtom(rateReviewAtom);
  const [selectedTerm, setSelectedTerm] = React.useState<{ id: string; term: TermEntry } | null>(null);

  const { view, activeCourseId, activeChapterId, activeStepId } = navigation;
  const courses = catalog.data?.index.courses ?? EMPTY_COURSES;
  const readyCourses = React.useMemo(() => courses.filter(isCourseReady), [courses]);
  const activeCourse = readyCourses.find((course) => course.courseId === activeCourseId) ?? readyCourses[0] ?? null;
  const activeChapter =
    activeCourse?.chapters.find((chapter) => chapter.chapterId === activeChapterId) ?? activeCourse?.chapters[0] ?? null;
  const manifests = catalog.data?.manifests ?? EMPTY_MANIFESTS;
  const activeManifest = activeCourse && activeChapter ? manifests[chapterKey(activeCourse.courseId, activeChapter.chapterId)] : undefined;
  const activeSteps = activeManifest?.steps ?? EMPTY_STEPS;
  const continueTarget = React.useMemo(() => getCourseContinueTarget(activeCourse, manifests, progress), [activeCourse, manifests, progress]);
  const story = useStoryBundle(activeCourse, activeChapter, activeStepId);
  const textbook = useTextbookState(view === "textbook" ? activeChapter : null);
  const practice = usePracticeIndex(activeCourse, manifests);
  const courseStats = React.useMemo(() => buildCourseStats(courses, manifests, progress), [courses, manifests, progress]);
  const dueCards = React.useMemo(() => countDueCards(practice, progress), [practice, progress]);

  const setNavigationState = React.useCallback(
    (patch: Partial<AppShellState>) => {
      patchNavigation(patch);
    },
    [patchNavigation]
  );

  const setView = React.useCallback(
    (nextView: ViewMode) => {
      setNavigationState({ view: nextView });
    },
    [setNavigationState]
  );

  React.useEffect(() => {
    if (!readyCourses.length) {
      if (activeCourseId !== null || activeChapterId !== null || activeStepId !== null) {
        setNavigationState({
          activeCourseId: null,
          activeChapterId: null,
          activeStepId: null
        });
      }
      return;
    }

    if (!activeCourseId || !readyCourses.some((course) => course.courseId === activeCourseId)) {
      setNavigationState({
        activeCourseId: readyCourses[0].courseId,
        activeChapterId: readyCourses[0].chapters[0]?.chapterId ?? null
      });
    }
  }, [activeChapterId, activeCourseId, activeStepId, readyCourses, setNavigationState]);

  React.useEffect(() => {
    if (!activeCourse) {
      return;
    }

    if (!activeChapterId || !activeCourse.chapters.some((chapter) => chapter.chapterId === activeChapterId)) {
      setNavigationState({
        activeChapterId: activeCourse.chapters[0]?.chapterId ?? null
      });
    }
  }, [activeChapterId, activeCourse, setNavigationState]);

  React.useEffect(() => {
    if (!activeCourse || !activeChapter) {
      return;
    }

    const key = chapterKey(activeCourse.courseId, activeChapter.chapterId);
    const remembered = progress.lastStepByChapter[key];
    const fallback = activeSteps[0]?.sequence_id ?? null;
    const nextStep = activeSteps.find((step) => step.sequence_id === remembered)?.sequence_id ?? fallback;
    if (nextStep !== activeStepId) {
      setNavigationState({
        activeStepId: nextStep
      });
    }
  }, [activeCourse, activeChapter, activeStepId, activeSteps, progress.lastStepByChapter, setNavigationState]);

  const openCourse = React.useCallback(
    (courseId: string) => {
      const course = readyCourses.find((entry) => entry.courseId === courseId);
      if (!course) {
        return;
      }
      setNavigationState({
        activeCourseId: courseId,
        activeChapterId: course.chapters[0]?.chapterId ?? null,
        view: "course"
      });
    },
    [readyCourses, setNavigationState]
  );

  const openChapter = React.useCallback(
    (chapterId: string) => {
      if (!activeCourse) {
        return;
      }
      const chapter = activeCourse.chapters.find((entry) => entry.chapterId === chapterId);
      if (!chapter) {
        return;
      }
      setNavigationState({
        activeChapterId: chapterId,
        view: "chapter"
      });
    },
    [activeCourse, setNavigationState]
  );

  const navigateToStep = React.useCallback(
    (chapterId: string, stepId: string, nextView: ViewMode = "learning") => {
      if (!activeCourse) {
        return;
      }
      const chapter = activeCourse.chapters.find((entry) => entry.chapterId === chapterId);
      if (!chapter) {
        return;
      }
      rememberStep({
        courseId: activeCourse.courseId,
        chapterId,
        stepId
      });
      setNavigationState({
        activeCourseId: activeCourse.courseId,
        activeChapterId: chapterId,
        activeStepId: stepId,
        view: nextView
      });
    },
    [activeCourse, rememberStep, setNavigationState]
  );

  const launchLearning = React.useCallback(
    (stepId?: string) => {
      if (!activeCourse || !activeChapter) {
        return;
      }
      const nextStepId = stepId ?? activeStepId ?? activeSteps[0]?.sequence_id;
      if (!nextStepId) {
        return;
      }
      navigateToStep(activeChapter.chapterId, nextStepId);
    },
    [activeChapter, activeCourse, activeStepId, activeSteps, navigateToStep]
  );

  const continueCourse = React.useCallback(() => {
    if (!continueTarget) {
      return;
    }
    navigateToStep(continueTarget.chapterId, continueTarget.stepId);
  }, [continueTarget, navigateToStep]);

  const selectChapterStep = React.useCallback(
    (stepId: string) => {
      if (!activeCourse || !activeChapter) {
        return;
      }
      rememberStep({
        courseId: activeCourse.courseId,
        chapterId: activeChapter.chapterId,
        stepId
      });
      setNavigationState({
        activeStepId: stepId
      });
    },
    [activeChapter, activeCourse, rememberStep, setNavigationState]
  );

  const storyComplete = React.useCallback(() => {
    if (!activeCourse || !activeChapter || !activeStepId) {
      return;
    }
    markStepComplete({
      courseId: activeCourse.courseId,
      chapterId: activeChapter.chapterId,
      stepId: activeStepId
    });
    const currentIndex = activeSteps.findIndex((step) => step.sequence_id === activeStepId);
    const nextStep = activeSteps[currentIndex + 1]?.sequence_id;
    if (nextStep) {
      navigateToStep(activeChapter.chapterId, nextStep);
      return;
    }
    setView("chapter");
  }, [activeChapter, activeCourse, activeStepId, activeSteps, markStepComplete, navigateToStep, setView]);

  if (catalog.loading) {
    return (
      <div className="content-shell">
        <LoadingState message="Loading course worlds..." />
      </div>
    );
  }

  if (catalog.error || !catalog.data) {
    return (
      <div className="content-shell">
        <EmptyState message={catalog.error ?? "No courses found."} tone="danger" />
      </div>
    );
  }

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand">
          <div className="brand-mark">
            <Compass size={16} />
          </div>
          <div className="brand-copy">
            <h1>Playdex Player</h1>
            <p>Learn like a route-based adventure, not a dashboard.</p>
          </div>
        </div>

        <div className="side-label">Navigation</div>
        <div className="nav-stack">
          <button className={`nav-button ${view === "library" ? "active" : ""}`} onClick={() => setView("library")}>
            <span className="nav-button-content">
              <Library size={16} />
              <span>Course Library</span>
            </span>
          </button>
          {activeCourse ? (
            <>
              <button className={`nav-button ${view === "course" ? "active" : ""}`} onClick={() => setView("course")}>
                <span className="nav-button-content">
                  <Home size={16} />
                  <span>Course Home</span>
                </span>
              </button>
              <button className={`nav-button ${view === "chapter" ? "active" : ""}`} onClick={() => setView("chapter")}>
                <span className="nav-button-content">
                  <MapIcon size={16} />
                  <span>Chapter Route</span>
                </span>
              </button>
              <button className={`nav-button ${view === "review" ? "active" : ""}`} onClick={() => setView("review")}>
                <span className="nav-button-content">
                  <Brain size={16} />
                  <span>Recall Center</span>
                </span>
              </button>
              <button className={`nav-button ${view === "bank" ? "active" : ""}`} onClick={() => setView("bank")}>
                <span className="nav-button-content">
                  <Archive size={16} />
                  <span>Vault</span>
                </span>
              </button>
            </>
          ) : null}
        </div>

        <div className="side-label">My Courses</div>
        <div className="course-list">
          {courses.map((course) => {
            const stats = courseStats.get(course.courseId);
            const isBlocked = !isCourseReady(course);
            return (
              <button
                key={course.courseId}
                className={`course-jump ${activeCourse?.courseId === course.courseId ? "active" : ""}`}
                disabled={isBlocked}
                title={isBlocked ? (course.validationErrors?.[0]?.message ?? "Course is blocked by backend catalog validation.") : undefined}
                onClick={() => openCourse(course.courseId)}
              >
                <div className="course-jump-title">{course.title}</div>
                <div className="course-jump-meta">
                  <span>{course.category ?? "Course"}</span>
                  <span>{isBlocked ? "blocked" : `${stats?.mastery ?? 0}% mastery`}</span>
                </div>
                <div className="progress-track" style={{ marginTop: 12 }}>
                  <div className="progress-fill" style={{ width: `${stats?.mastery ?? 0}%` }} />
                </div>
              </button>
            );
          })}
        </div>
      </aside>

      <div className="main-stage">
        <header className="topbar">
          <div className="topbar-copy">
            <span className="eyebrow">Now Exploring</span>
            <h2>{activeCourse?.title ?? "Course Library"}</h2>
            <p>{activeChapter ? `${activeChapter.lessonId} · ${activeChapter.title}` : "Pick a course and continue the route."}</p>
          </div>
          <div className="topbar-actions">
            <button className="pill-button ghost" onClick={() => setView("library")}>
              <span className="button-label">
                <Search size={15} />
                <span>Search</span>
              </span>
            </button>
            {activeCourse ? (
              <span className="pill accent">
                <Clock3 size={13} />
                <span>{dueCards} cards to revisit</span>
              </span>
            ) : null}
            {activeCourse ? (
              <button className="pill-button ghost" onClick={() => setView("course")}>
                <span className="button-label">
                  <Home size={15} />
                  <span>Return Home</span>
                </span>
              </button>
            ) : null}
            <div className="user-avatar">
              <User size={15} />
            </div>
          </div>
        </header>

        <main className="content-shell">
          {view === "library" ? <LibraryView courses={courses} courseStats={courseStats} onOpenCourse={openCourse} /> : null}

          {view !== "library" && !activeCourse ? (
            <EmptyState message="No ready courses are available. Open Course Library to inspect backend validation errors." tone="danger" />
          ) : null}

          {view === "course" && activeCourse ? (
            <CourseHomeView
              course={activeCourse}
              manifests={manifests}
              courseStats={courseStats.get(activeCourse.courseId)}
              dueCards={dueCards}
              progress={progress}
              onContinueRoute={continueCourse}
              onOpenChapter={openChapter}
              onOpenReview={() => setView("review")}
              onOpenVault={() => setView("bank")}
            />
          ) : null}

          {view === "chapter" && activeCourse && activeChapter ? (
            <ChapterHomeView
              course={activeCourse}
              chapter={activeChapter}
              manifest={activeManifest}
              progress={progress}
              activeStepId={activeStepId}
              onStartStory={launchLearning}
              onOpenTextbook={() => setView("textbook")}
              onSelectStep={selectChapterStep}
            />
          ) : null}

          {view === "review" && activeCourse ? (
            <ReviewView
              course={activeCourse}
              practice={practice}
              reviewHistory={progress.reviewHistory}
              onRate={(questionId, rating) => rateReview({ questionId, rating })}
            />
          ) : null}

          {view === "bank" && activeCourse ? <VaultView course={activeCourse} practice={practice} /> : null}

          {view === "textbook" && activeCourse && activeChapter ? (
            <TextbookView chapter={activeChapter} textbook={textbook} practice={practice.data ?? null} />
          ) : null}
        </main>

        {activeCourse ? (
          <LearningDock
            course={activeCourse}
            chapter={activeChapter}
            dueCards={dueCards}
            onHome={() => setView("course")}
            onContinue={continueCourse}
            onReview={() => setView("review")}
            onVault={() => setView("bank")}
            canContinue={Boolean(continueTarget)}
          />
        ) : null}

        {view === "learning" && activeCourse && activeChapter ? (
          <StoryOverlay
            key={`${activeChapter.chapterId}:${activeStepId ?? "none"}`}
            course={activeCourse}
            chapter={activeChapter}
            stepMeta={activeSteps.find((step) => step.sequence_id === activeStepId) ?? null}
            bundle={story}
            onClose={() => setView("chapter")}
            onComplete={storyComplete}
            onOpenTerm={(id, term) => setSelectedTerm({ id, term })}
          />
        ) : null}

        {selectedTerm ? <TermDrawer selectedTerm={selectedTerm} onClose={() => setSelectedTerm(null)} /> : null}
      </div>
    </div>
  );
}
