import React from "react";
import ReactDOM from "react-dom/client";
import { evaluate } from "@mdx-js/mdx";
import * as runtime from "react/jsx-runtime";
import remarkGfm from "remark-gfm";
import {
  loadCourseIndex,
  loadManifest,
  loadQuestionBank,
  loadStep,
  loadTextbook
} from "./data";
import type {
  ChapterRecord,
  CourseIndex,
  CourseRecord,
  PlayerProgress,
  QuestionBank,
  QuestionFamily,
  QuestionIndex,
  QuestionVariant,
  ReviewHistoryEntry,
  StoryManifest,
  StoryManifestStep,
  StoryScreen,
  StoryStep,
  TermEntry
} from "./types";
import "./styles.css";

type ViewMode = "library" | "course" | "chapter" | "learning" | "review" | "bank" | "textbook";

type AsyncState<T> = {
  loading: boolean;
  data: T | null;
  error: string | null;
};

type CatalogState = AsyncState<{
  index: CourseIndex;
  manifests: Record<string, StoryManifest>;
}>;

type PracticeState = AsyncState<QuestionIndex>;

type TextbookState = AsyncState<string>;

type CourseSummary = {
  totalSteps: number;
  completedSteps: number;
  mastery: number;
  chapters: Array<{
    chapterId: string;
    totalSteps: number;
    completedSteps: number;
    completionRatio: number;
  }>;
};

const emptyState = <T,>(): AsyncState<T> => ({ loading: true, data: null, error: null });

const PROGRESS_STORAGE_KEY = "playdex-player-progress-v1";

const defaultProgress = (): PlayerProgress => ({
  completedSteps: {},
  lastStepByChapter: {},
  reviewHistory: {}
});

const chapterKey = (courseId: string, chapterId: string) => `${courseId}:${chapterId}`;

const safeColor = (value?: string, fallback = "#6ee7b7") => value ?? fallback;

function loadProgressFromStorage(): PlayerProgress {
  try {
    const raw = window.localStorage.getItem(PROGRESS_STORAGE_KEY);
    if (!raw) {
      return defaultProgress();
    }
    const parsed = JSON.parse(raw) as PlayerProgress;
    return {
      completedSteps: parsed.completedSteps ?? {},
      lastStepByChapter: parsed.lastStepByChapter ?? {},
      reviewHistory: parsed.reviewHistory ?? {}
    };
  } catch {
    return defaultProgress();
  }
}

function usePlayerProgress() {
  const [progress, setProgress] = React.useState<PlayerProgress>(() => loadProgressFromStorage());

  React.useEffect(() => {
    window.localStorage.setItem(PROGRESS_STORAGE_KEY, JSON.stringify(progress));
  }, [progress]);

  const markStepComplete = React.useCallback((courseId: string, chapterId: string, stepId: string) => {
    const key = chapterKey(courseId, chapterId);
    setProgress((current) => {
      const completed = new Set(current.completedSteps[key] ?? []);
      completed.add(stepId);
      return {
        ...current,
        completedSteps: {
          ...current.completedSteps,
          [key]: [...completed]
        },
        lastStepByChapter: {
          ...current.lastStepByChapter,
          [key]: stepId
        }
      };
    });
  }, []);

  const rememberStep = React.useCallback((courseId: string, chapterId: string, stepId: string) => {
    const key = chapterKey(courseId, chapterId);
    setProgress((current) => ({
      ...current,
      lastStepByChapter: {
        ...current.lastStepByChapter,
        [key]: stepId
      }
    }));
  }, []);

  const rateReview = React.useCallback((questionId: string, rating: number) => {
    setProgress((current) => {
      const previous = current.reviewHistory[questionId];
      const nextEntry: ReviewHistoryEntry = {
        seenCount: (previous?.seenCount ?? 0) + 1,
        lastRating: rating,
        updatedAt: Date.now()
      };
      return {
        ...current,
        reviewHistory: {
          ...current.reviewHistory,
          [questionId]: nextEntry
        }
      };
    });
  }, []);

  return { progress, markStepComplete, rememberStep, rateReview };
}

function useCatalogState(): CatalogState {
  const [state, setState] = React.useState<CatalogState>(emptyState);

  React.useEffect(() => {
    let alive = true;
    setState(emptyState());

    loadCourseIndex().then(async (result) => {
      if (!result.data) {
        if (alive) {
          setState({ loading: false, data: null, error: result.error ?? "Failed to load course index." });
        }
        return;
      }

      const manifestPairs = await Promise.all(
        result.data.courses.flatMap((course) =>
          course.chapters.map(async (chapter) => {
            const manifest = await loadManifest(chapter);
            return [chapterKey(course.courseId, chapter.chapterId), manifest.data ?? {}] as const;
          })
        )
      );

      if (alive) {
        setState({
          loading: false,
          data: {
            index: result.data,
            manifests: Object.fromEntries(manifestPairs)
          },
          error: null
        });
      }
    });

    return () => {
      alive = false;
    };
  }, []);

  return state;
}

function usePracticeIndex(course: CourseRecord | null, manifests: Record<string, StoryManifest>): PracticeState {
  const [state, setState] = React.useState<PracticeState>({ loading: false, data: null, error: null });

  React.useEffect(() => {
    if (!course) {
      setState({ loading: false, data: null, error: null });
      return;
    }

    let alive = true;
    setState(emptyState());

    Promise.all(
      course.chapters.flatMap((chapter) => {
        const manifest = manifests[chapterKey(course.courseId, chapter.chapterId)];
        const steps = manifest.steps ?? [];
        return steps.map(async (step) => {
          const bank = await loadQuestionBank(chapter, step.sequence_id ?? "step1");
          return { bank: bank.data, chapter, stepId: step.sequence_id ?? "step1" };
        });
      })
    ).then((results) => {
      if (!alive) {
        return;
      }

      const families = new Map<string, QuestionFamily>();
      const questions = new Map<string, { family: QuestionFamily; variant: QuestionVariant }>();
      const flashcards: QuestionIndex["flashcards"] = [];
      const quizFamilies: QuestionFamily[] = [];
      const longformFamilies: QuestionFamily[] = [];
      const coverage = [];

      for (const entry of results) {
        const bank = entry.bank;
        if (!bank) {
          continue;
        }

        for (const item of bank.coverage_map ?? []) {
          coverage.push(item);
        }

        for (const family of bank.flashcard_families ?? []) {
          if (family.family_id) {
            families.set(family.family_id, family);
          }
          for (const variant of family.variants ?? []) {
            if (variant.question_id) {
              questions.set(variant.question_id, { family, variant });
            }
            flashcards.push({
              family,
              variant,
              chapterId: entry.chapter.chapterId,
              chapterTitle: entry.chapter.title,
              stepId: entry.stepId
            });
          }
        }

        for (const family of bank.quiz_families ?? []) {
          if (family.family_id) {
            families.set(family.family_id, family);
          }
          quizFamilies.push(family);
          for (const variant of family.variants ?? []) {
            if (variant.question_id) {
              questions.set(variant.question_id, { family, variant });
            }
          }
        }

        for (const family of bank.longform_families ?? []) {
          if (family.family_id) {
            families.set(family.family_id, family);
          }
          longformFamilies.push(family);
          for (const variant of family.variants ?? []) {
            if (variant.question_id) {
              questions.set(variant.question_id, { family, variant });
            }
          }
        }
      }

      setState({
        loading: false,
        data: { families, questions, flashcards, quizFamilies, longformFamilies, coverage },
        error: null
      });
    });

    return () => {
      alive = false;
    };
  }, [course, manifests]);

  return state;
}

function useStoryBundle(course: CourseRecord | null, chapter: ChapterRecord | null, stepId: string | null) {
  const [state, setState] = React.useState<AsyncState<{ step: StoryStep; bank: QuestionBank }>>({
    loading: false,
    data: null,
    error: null
  });

  React.useEffect(() => {
    if (!course || !chapter || !stepId) {
      setState({ loading: false, data: null, error: null });
      return;
    }

    let alive = true;
    setState(emptyState());
    Promise.all([loadStep(chapter, stepId), loadQuestionBank(chapter, stepId)]).then(([step, bank]) => {
      if (!alive) {
        return;
      }
      if (!step.data || !bank.data) {
        setState({
          loading: false,
          data: null,
          error: step.error ?? bank.error ?? `Failed to load ${stepId}.`
        });
        return;
      }
      setState({
        loading: false,
        data: { step: step.data, bank: bank.data },
        error: null
      });
    });

    return () => {
      alive = false;
    };
  }, [course, chapter, stepId]);

  return state;
}

function useTextbookState(chapter: ChapterRecord | null): TextbookState {
  const [state, setState] = React.useState<TextbookState>({ loading: false, data: null, error: null });

  React.useEffect(() => {
    if (!chapter) {
      setState({ loading: false, data: null, error: null });
      return;
    }
    let alive = true;
    setState(emptyState());
    loadTextbook(chapter).then((result) => {
      if (alive) {
        setState({
          loading: false,
          data: result.data,
          error: result.error
        });
      }
    });
    return () => {
      alive = false;
    };
  }, [chapter]);

  return state;
}

function normalizeMdxSource(source: string) {
  return source.replace(/^---[\s\S]*?---\s*/, "").replace(/^(#{1,6}\s+.+?)\s+\{#[^}]+}\s*$/gm, "$1");
}

function flattenText(value: React.ReactNode): string {
  if (value == null || typeof value === "boolean") {
    return "";
  }
  if (typeof value === "string" || typeof value === "number") {
    return String(value);
  }
  if (Array.isArray(value)) {
    return value.map(flattenText).join("");
  }
  if (React.isValidElement(value)) {
    const element = value as React.ReactElement<{ children?: React.ReactNode }>;
    return flattenText(element.props.children);
  }
  return "";
}

function useCompiledMdx(source: string | null) {
  const [state, setState] = React.useState<AsyncState<React.ComponentType<{ components?: Record<string, React.ComponentType<any>> }>>>({
    loading: false,
    data: null,
    error: null
  });

  React.useEffect(() => {
    if (!source) {
      setState({ loading: false, data: null, error: null });
      return;
    }
    let alive = true;
    setState(emptyState());
    evaluate(normalizeMdxSource(source), {
      ...runtime,
      remarkPlugins: [remarkGfm]
    })
      .then((compiled) => {
        if (alive) {
          setState({
            loading: false,
            data: (compiled as { default: React.ComponentType<any> }).default,
            error: null
          });
        }
      })
      .catch((error) => {
        if (alive) {
          setState({
            loading: false,
            data: null,
            error: error instanceof Error ? error.message : String(error)
          });
        }
      });
    return () => {
      alive = false;
    };
  }, [source]);

  return state;
}

function App() {
  const catalog = useCatalogState();
  const { progress, markStepComplete, rememberStep, rateReview } = usePlayerProgress();
  const [view, setView] = React.useState<ViewMode>("library");
  const [activeCourseId, setActiveCourseId] = React.useState<string | null>(null);
  const [activeChapterId, setActiveChapterId] = React.useState<string | null>(null);
  const [activeStepId, setActiveStepId] = React.useState<string | null>(null);
  const [selectedTerm, setSelectedTerm] = React.useState<{ id: string; term: TermEntry } | null>(null);

  const courses = catalog.data?.index.courses ?? [];
  const activeCourse = courses.find((course) => course.courseId === activeCourseId) ?? courses[0] ?? null;
  const activeChapter =
    activeCourse?.chapters.find((chapter) => chapter.chapterId === activeChapterId) ?? activeCourse?.chapters[0] ?? null;

  const manifests = catalog.data?.manifests ?? {};
  const activeManifest = activeCourse && activeChapter ? manifests[chapterKey(activeCourse.courseId, activeChapter.chapterId)] : undefined;
  const activeSteps = activeManifest?.steps ?? [];
  const story = useStoryBundle(activeCourse, activeChapter, activeStepId);
  const textbook = useTextbookState(view === "textbook" ? activeChapter : null);
  const practice = usePracticeIndex(activeCourse, manifests);

  React.useEffect(() => {
    if (!courses.length) {
      return;
    }
    if (!activeCourseId || !courses.some((course) => course.courseId === activeCourseId)) {
      setActiveCourseId(courses[0].courseId);
      setActiveChapterId(courses[0].chapters[0]?.chapterId ?? null);
    }
  }, [courses, activeCourseId]);

  React.useEffect(() => {
    if (!activeCourse) {
      return;
    }
    if (!activeChapterId || !activeCourse.chapters.some((chapter) => chapter.chapterId === activeChapterId)) {
      setActiveChapterId(activeCourse.chapters[0]?.chapterId ?? null);
    }
  }, [activeCourse, activeChapterId]);

  React.useEffect(() => {
    if (!activeCourse || !activeChapter) {
      return;
    }
    const key = chapterKey(activeCourse.courseId, activeChapter.chapterId);
    const remembered = progress.lastStepByChapter[key];
    const fallback = activeSteps[0]?.sequence_id ?? null;
    const nextStep = activeSteps.find((step) => step.sequence_id === remembered)?.sequence_id ?? fallback;
    if (nextStep && nextStep !== activeStepId) {
      setActiveStepId(nextStep);
    }
  }, [activeCourse, activeChapter, activeSteps, progress.lastStepByChapter, activeStepId]);

  const courseStats = React.useMemo(() => {
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
  }, [courses, manifests, progress.completedSteps]);

  const dueCards = React.useMemo(() => {
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
  }, [practice.data, progress.reviewHistory]);

  const openCourse = React.useCallback((courseId: string) => {
    const course = courses.find((entry) => entry.courseId === courseId);
    if (!course) {
      return;
    }
    setActiveCourseId(courseId);
    setActiveChapterId(course.chapters[0]?.chapterId ?? null);
    setView("course");
  }, [courses]);

  const openChapter = React.useCallback((chapterId: string) => {
    if (!activeCourse) {
      return;
    }
    const chapter = activeCourse.chapters.find((entry) => entry.chapterId === chapterId);
    if (!chapter) {
      return;
    }
    setActiveChapterId(chapterId);
    setView("chapter");
  }, [activeCourse]);

  const launchLearning = React.useCallback(
    (stepId?: string) => {
      if (!activeCourse || !activeChapter) {
        return;
      }
      const nextStepId = stepId ?? activeStepId ?? activeSteps[0]?.sequence_id;
      if (!nextStepId) {
        return;
      }
      rememberStep(activeCourse.courseId, activeChapter.chapterId, nextStepId);
      setActiveStepId(nextStepId);
      setView("learning");
    },
    [activeCourse, activeChapter, activeStepId, activeSteps, rememberStep]
  );

  const storyComplete = React.useCallback(() => {
    if (!activeCourse || !activeChapter || !activeStepId) {
      return;
    }
    markStepComplete(activeCourse.courseId, activeChapter.chapterId, activeStepId);
    const currentIndex = activeSteps.findIndex((step) => step.sequence_id === activeStepId);
    const nextStep = activeSteps[currentIndex + 1]?.sequence_id;
    if (nextStep) {
      setActiveStepId(nextStep);
      rememberStep(activeCourse.courseId, activeChapter.chapterId, nextStep);
      return;
    }
    setView("chapter");
  }, [activeCourse, activeChapter, activeStepId, activeSteps, markStepComplete, rememberStep]);

  if (catalog.loading) {
    return <div className="content-shell"><div className="loading-state">Loading course worlds...</div></div>;
  }

  if (catalog.error || !catalog.data) {
    return <div className="content-shell"><div className="empty-state">{catalog.error ?? "No courses found."}</div></div>;
  }

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand">
          <div className="brand-mark">PX</div>
          <div className="brand-copy">
            <h1>Playdex Player</h1>
            <p>Learn like a route-based adventure, not a dashboard.</p>
          </div>
        </div>

        <div className="side-label">Navigation</div>
        <div className="nav-stack">
          <button className={`nav-button ${view === "library" ? "active" : ""}`} onClick={() => setView("library")}>
            Course Library
          </button>
          {activeCourse ? (
            <>
              <button className={`nav-button ${view === "course" ? "active" : ""}`} onClick={() => setView("course")}>
                Course Home
              </button>
              <button className={`nav-button ${view === "chapter" ? "active" : ""}`} onClick={() => setView("chapter")}>
                Chapter Route
              </button>
              <button className={`nav-button ${view === "review" ? "active" : ""}`} onClick={() => setView("review")}>
                Recall Center
              </button>
              <button className={`nav-button ${view === "bank" ? "active" : ""}`} onClick={() => setView("bank")}>
                Vault
              </button>
            </>
          ) : null}
        </div>

        <div className="side-label">My Courses</div>
        <div className="course-list">
          {courses.map((course) => {
            const stats = courseStats.get(course.courseId);
            return (
              <button
                key={course.courseId}
                className={`course-jump ${activeCourse?.courseId === course.courseId ? "active" : ""}`}
                onClick={() => openCourse(course.courseId)}
              >
                <div className="course-jump-title">{course.title}</div>
                <div className="course-jump-meta">
                  <span>{course.category ?? "Course"}</span>
                  <span>{stats?.mastery ?? 0}% mastery</span>
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
            <p>
              {activeChapter ? `${activeChapter.lessonId} · ${activeChapter.title}` : "Pick a course and continue the route."}
            </p>
          </div>
          <div className="topbar-actions">
            <button className="pill-button ghost" onClick={() => setView("library")}>
              Search
            </button>
            {activeCourse ? <span className="pill accent">{dueCards} cards to revisit</span> : null}
            {activeCourse ? (
              <button className="pill-button ghost" onClick={() => setView("course")}>
                Return Home
              </button>
            ) : null}
            <div className="user-avatar">U</div>
          </div>
        </header>

        <main className="content-shell">
          {view === "library" ? (
            <LibraryView courses={courses} courseStats={courseStats} onOpenCourse={openCourse} />
          ) : null}

          {view === "course" && activeCourse ? (
            <CourseHomeView
              course={activeCourse}
              manifests={manifests}
              courseStats={courseStats.get(activeCourse.courseId)}
              dueCards={dueCards}
              progress={progress}
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
              onSelectStep={(stepId) => {
                setActiveStepId(stepId);
                rememberStep(activeCourse.courseId, activeChapter.chapterId, stepId);
              }}
            />
          ) : null}

          {view === "review" && activeCourse ? (
            <ReviewView
              course={activeCourse}
              practice={practice}
              reviewHistory={progress.reviewHistory}
              onRate={rateReview}
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
            onContinue={() => launchLearning()}
            onReview={() => setView("review")}
            onVault={() => setView("bank")}
          />
        ) : null}

        {view === "learning" && activeCourse && activeChapter ? (
          <StoryOverlay
            key={`${activeChapter.chapterId}:${activeStepId ?? "none"}`}
            course={activeCourse}
            chapter={activeChapter}
            manifest={activeManifest}
            stepMeta={activeSteps.find((step) => step.sequence_id === activeStepId) ?? null}
            bundle={story}
            onClose={() => setView("chapter")}
            onComplete={storyComplete}
            onOpenTerm={(id, term) => setSelectedTerm({ id, term })}
          />
        ) : null}

        {selectedTerm ? (
          <TermDrawer selectedTerm={selectedTerm} onClose={() => setSelectedTerm(null)} />
        ) : null}
      </div>
    </div>
  );
}

function LibraryView({
  courses,
  courseStats,
  onOpenCourse
}: {
  courses: CourseRecord[];
  courseStats: Map<string, CourseSummary>;
  onOpenCourse: (courseId: string) => void;
}) {
  return (
    <section>
      <div className="section-heading">
        <div>
          <h2>Pick up your game</h2>
          <p>Choose a course world, then move through route scenes, recall loops, and the vault instead of browsing raw assets.</p>
        </div>
      </div>
      <div className="library-grid">
        {courses.map((course) => {
          const stats = courseStats.get(course.courseId);
          const color = safeColor(course.brandColor);
          return (
            <article
              key={course.courseId}
              className="course-card"
              style={{ ["--card-accent" as string]: `${color}22` }}
              onClick={() => onOpenCourse(course.courseId)}
            >
              <div className="course-card-head">
                <span className="pill">{course.category ?? "Course"}</span>
                <div className="course-emblem" style={{ background: `linear-gradient(135deg, ${color}, #60a5fa)` }}>
                  {course.courseId.slice(0, 2).toUpperCase()}
                </div>
                <h3>{course.title}</h3>
                <p>{course.chapters.length} chapters · {stats?.totalSteps ?? 0} scenes · progression tracked locally</p>
              </div>
              <div className="course-card-footer">
                <div className="metric-row">
                  <span className="pill accent">{stats?.mastery ?? 0}% mastery</span>
                  <span className="pill">{stats?.completedSteps ?? 0} scenes cleared</span>
                </div>
                <div className="progress-track">
                  <div className="progress-fill" style={{ width: `${stats?.mastery ?? 0}%` }} />
                </div>
              </div>
            </article>
          );
        })}
      </div>
    </section>
  );
}

function CourseHomeView({
  course,
  manifests,
  courseStats,
  dueCards,
  progress,
  onOpenChapter,
  onOpenReview,
  onOpenVault
}: {
  course: CourseRecord;
  manifests: Record<string, StoryManifest>;
  courseStats?: CourseSummary;
  dueCards: number;
  progress: PlayerProgress;
  onOpenChapter: (chapterId: string) => void;
  onOpenReview: () => void;
  onOpenVault: () => void;
}) {
  const color = safeColor(course.brandColor);
  return (
    <section>
      <div className="hero-panel" style={{ ["--card-accent" as string]: `${color}26` }}>
        <div className="hero-copy">
          <span className="pill accent">Verified Course Game</span>
          <h1>{course.title}</h1>
          <p>
            Enter the learner home base for this course: route selection, immersive scene play, recall loops, and a vault of generated challenges.
          </p>
          <div className="hero-actions" style={{ marginTop: 22 }}>
            <button className="pill-button primary" onClick={() => onOpenChapter(course.chapters[0]?.chapterId ?? "")}>
              Continue Route
            </button>
            <button className="pill-button" onClick={onOpenReview}>
              Recall Center
            </button>
            <button className="pill-button ghost" onClick={onOpenVault}>
              Open Vault
            </button>
          </div>
        </div>
        <div className="hero-art" style={{ background: `linear-gradient(135deg, ${color}, #60a5fa)` }}>
          {course.courseId.toUpperCase()}
        </div>
      </div>

      <div className="metric-grid" style={{ marginTop: 20 }}>
        <div className="metric-card">
          <strong>{courseStats?.mastery ?? 0}%</strong>
          <p>Route mastery across completed scenes.</p>
        </div>
        <div className="metric-card">
          <strong>{courseStats?.completedSteps ?? 0}/{courseStats?.totalSteps ?? 0}</strong>
          <p>Guided scenes cleared so far.</p>
        </div>
        <div className="metric-card">
          <strong>{dueCards}</strong>
          <p>Cards currently worth revisiting in recall mode.</p>
        </div>
      </div>

          <div className="section-heading" style={{ marginTop: 28 }}>
        <div>
          <h3>Choose your route</h3>
          <p>Every chapter ships with a story route, a reading room, and downstream practice families.</p>
        </div>
      </div>
      <div className="chapter-grid">
        {course.chapters.map((chapter) => {
          const manifest = manifests[chapterKey(course.courseId, chapter.chapterId)];
          const completed = progress.completedSteps[chapterKey(course.courseId, chapter.chapterId)] ?? [];
          const totalSteps = manifest?.steps?.length ?? 0;
          const progressWidth = totalSteps === 0 ? 0 : Math.round((completed.length / totalSteps) * 100);
          return (
            <article
              key={chapter.chapterId}
              className="chapter-card"
              style={{ ["--card-accent" as string]: `${color}1f` }}
              onClick={() => onOpenChapter(chapter.chapterId)}
            >
              <div className="chapter-card-head">
                <span className="pill">{chapter.lessonId}</span>
                <h4>{chapter.title}</h4>
                <p>{totalSteps} scenes · {completed.length} cleared · story + recall + textbook</p>
              </div>
              <div className="progress-track" style={{ marginTop: 16 }}>
                <div className="progress-fill" style={{ width: `${progressWidth}%` }} />
              </div>
            </article>
          );
        })}
      </div>
    </section>
  );
}

function ChapterHomeView({
  course,
  chapter,
  manifest,
  progress,
  activeStepId,
  onStartStory,
  onOpenTextbook,
  onSelectStep
}: {
  course: CourseRecord;
  chapter: ChapterRecord;
  manifest?: StoryManifest;
  progress: PlayerProgress;
  activeStepId: string | null;
  onStartStory: (stepId?: string) => void;
  onOpenTextbook: () => void;
  onSelectStep: (stepId: string) => void;
}) {
  const steps = manifest?.steps ?? [];
  const completed = progress.completedSteps[chapterKey(course.courseId, chapter.chapterId)] ?? [];
  const completionRatio = steps.length === 0 ? 0 : Math.round((completed.length / steps.length) * 100);

  return (
    <section className="chapter-detail">
      <div className="panel">
        <div className="chapter-copy">
          <span className="pill accent">{chapter.lessonId} Route</span>
          <h2 className="chapter-title" style={{ fontSize: "clamp(2.3rem, 5vw, 4.2rem)", lineHeight: 0.98 }}>
            {chapter.title}
          </h2>
          <p>
            Start the route from the opening scene, or jump back to the exact concept node where you stopped.
          </p>
        </div>

        <div className="hero-actions" style={{ marginTop: 22 }}>
          <button className="pill-button primary" onClick={() => onStartStory(activeStepId ?? steps[0]?.sequence_id)}>
            Start Route
          </button>
          <button className="pill-button" onClick={onOpenTextbook}>
            Read Textbook
          </button>
        </div>

        <div className="metric-grid" style={{ marginTop: 24 }}>
          <div className="metric-card">
            <strong>{steps.length}</strong>
            <p>Guided scenes in this chapter route.</p>
          </div>
          <div className="metric-card">
            <strong>{completionRatio}%</strong>
            <p>Completion rate for the current route.</p>
          </div>
          <div className="metric-card">
            <strong>{completed.length}</strong>
            <p>Scenes already cleared and available for replay.</p>
          </div>
        </div>
      </div>

      <div className="panel">
        <div className="section-heading" style={{ marginBottom: 16 }}>
          <div>
            <h3>Scene Select</h3>
            <p>Pick a concept and jump straight into that route node.</p>
          </div>
        </div>

        <div className="step-list">
          {steps.map((step, index) => {
            const stepId = step.sequence_id ?? `step-${index + 1}`;
            const isDone = completed.includes(stepId);
            return (
              <button
                key={stepId}
                className={`step-row ${activeStepId === stepId ? "active" : ""}`}
                onClick={() => onSelectStep(stepId)}
              >
                <div className="step-index">{String(index + 1).padStart(2, "0")}</div>
                <div className="step-copy">
                  <strong>{step.concept ?? stepId}</strong>
                  <span>{isDone ? "Cleared once already" : "New route scene"}</span>
                </div>
                <span className="micro-pill">{isDone ? "Done" : "Ready"}</span>
              </button>
            );
          })}
        </div>
      </div>
    </section>
  );
}

function StoryOverlay({
  course,
  chapter,
  manifest,
  stepMeta,
  bundle,
  onClose,
  onComplete,
  onOpenTerm
}: {
  course: CourseRecord;
  chapter: ChapterRecord;
  manifest?: StoryManifest;
  stepMeta: StoryManifestStep | null;
  bundle: AsyncState<{ step: StoryStep; bank: QuestionBank }>;
  onClose: () => void;
  onComplete: () => void;
  onOpenTerm: (id: string, term: TermEntry) => void;
}) {
  const [screenIndex, setScreenIndex] = React.useState(0);
  const [showLog, setShowLog] = React.useState(false);
  const [selectedAnswer, setSelectedAnswer] = React.useState<number | null>(null);
  const [answered, setAnswered] = React.useState(false);

  React.useEffect(() => {
    setScreenIndex(0);
    setShowLog(false);
    setSelectedAnswer(null);
    setAnswered(false);
  }, [stepMeta?.sequence_id]);

  if (bundle.loading || !bundle.data) {
    return (
      <div className="story-overlay">
        <div className="overlay-layer">
          <div className="dialog-shell">
            <div className="loading-state">{bundle.error ?? "Loading route scene..."}</div>
          </div>
        </div>
      </div>
    );
  }

  const screens = bundle.data.step.screens ?? [];
  const current = screens[screenIndex];
  const progress = screens.length === 0 ? 0 : ((screenIndex + 1) / screens.length) * 100;
  const history = screens.slice(0, screenIndex);

  const handleNext = () => {
    if (!current) {
      return;
    }
    if (current.type === "exercise" && !answered) {
      return;
    }
    if (screenIndex < screens.length - 1) {
      setScreenIndex((value) => value + 1);
      setSelectedAnswer(null);
      setAnswered(false);
      return;
    }
    onComplete();
  };

  const handlePrev = () => {
    if (screenIndex === 0) {
      return;
    }
    setScreenIndex((value) => value - 1);
    setSelectedAnswer(null);
    setAnswered(false);
  };

  React.useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.code !== "Space") {
        return;
      }
      if (event.metaKey || event.ctrlKey || event.altKey) {
        return;
      }

      const target = event.target;
      if (
        target instanceof HTMLInputElement ||
        target instanceof HTMLTextAreaElement ||
        target instanceof HTMLSelectElement ||
        target instanceof HTMLButtonElement ||
        (target instanceof HTMLElement && target.isContentEditable)
      ) {
        return;
      }

      event.preventDefault();
      handleNext();
    };

    window.addEventListener("keydown", onKeyDown);
    return () => {
      window.removeEventListener("keydown", onKeyDown);
    };
  }, [handleNext]);

  const parseLines = (line: string, index: number) => {
    const parts = line.split(/(<term id="[^"]+">.*?<\/term>)/g);
    return (
      <p key={`${current?.id ?? "screen"}-${index}`}>
        {parts.map((part, partIndex) => {
          const match = part.match(/<term id="([^"]+)">([\s\S]*?)<\/term>/);
          if (!match) {
            return <React.Fragment key={partIndex}>{part}</React.Fragment>;
          }
          const [, id, label] = match;
          const term = bundle.data?.step.term_catalog?.[id];
          return (
            <span
              key={partIndex}
              className="term-token"
              onClick={(event) => {
                event.stopPropagation();
                if (term) {
                  onOpenTerm(id, term);
                }
              }}
            >
              {label}
            </span>
          );
        })}
      </p>
    );
  };

  const answerIndex =
    typeof current?.exercise?.answer === "number" ? current.exercise.answer : Number.NaN;

  return (
    <div className="story-overlay">
      <div className="overlay-layer">
        <div className="overlay-controls">
          <div className="overlay-title">
            <span>{course.title}</span>
            <strong>{chapter.lessonId} · {stepMeta?.concept ?? stepMeta?.sequence_id ?? "Route Scene"}</strong>
          </div>
          <div className="topbar-actions">
            <button className="pill-button ghost" onClick={() => setShowLog(true)}>
              Scene Log
            </button>
            <button className="pill-button danger" onClick={onClose}>
              Exit Route
            </button>
          </div>
        </div>

        <div className="scene-stage">
          {current?.type === "exercise" ? (
            <div className="scene-exercise">
              <span className="pill accent">Checkpoint</span>
              <h3>{current.exercise?.prompt ?? current.lines?.[0] ?? "Quick check"}</h3>
              <p className="story-note">Answer first, then continue. This keeps the story from becoming passive scrolling.</p>
              <div className="exercise-options">
                {(current.exercise?.options ?? []).map((option, index) => {
                  let stateClass = "";
                  if (answered && index === answerIndex) {
                    stateClass = "correct";
                  } else if (answered && index === selectedAnswer && index !== answerIndex) {
                    stateClass = "wrong";
                  }
                  return (
                    <button
                      key={option}
                      className={`exercise-option ${stateClass}`.trim()}
                      onClick={() => {
                        if (answered) {
                          return;
                        }
                        setSelectedAnswer(index);
                        setAnswered(true);
                      }}
                    >
                      {option}
                    </button>
                  );
                })}
              </div>
              {answered ? (
                <p className="story-note">{current.exercise?.explanation ?? "Checkpoint complete. Continue the route."}</p>
              ) : null}
            </div>
          ) : (
            <div className="scene-avatar">✦</div>
          )}
        </div>

        <div className="dialog-shell" onClick={current?.type === "exercise" ? undefined : handleNext}>
          <div className="dialog-topline">
            <span className="dialog-speaker">{current?.type === "exercise" ? "System Test" : "Instructor"}</span>
            <div className="dialog-progress">
              <div className="progress-fill" style={{ width: `${progress}%` }} />
            </div>
          </div>

          <div className="dialog-lines">
            {(current?.lines ?? ["No scene content found."]).map((line, index) => parseLines(line, index))}
          </div>

          <div className="story-footer">
            <div className="topbar-actions">
              <button className="pill-button ghost" onClick={handlePrev}>
                Back
              </button>
            </div>
          <div>
              {current?.type === "exercise"
                ? answered
                  ? "Continue"
                  : "Answer to Continue"
                : "Tap Anywhere to Continue"}
          </div>
            <div className="topbar-actions">
              <button className="pill-button primary" onClick={handleNext}>
                {screenIndex === screens.length - 1 ? "Finish Step" : "Next"}
              </button>
            </div>
          </div>
        </div>
      </div>

      {showLog ? (
        <div className="scrim" onClick={() => setShowLog(false)}>
          <aside className="story-log-panel" onClick={(event) => event.stopPropagation()}>
            <div className="drawer-head">
              <h3>Narrative Log</h3>
              <button className="pill-button ghost" onClick={() => setShowLog(false)}>
                Close
              </button>
            </div>
            <div className="log-list">
              {history.length === 0 ? <div className="empty-state">No prior lines yet.</div> : null}
              {history.map((screen, index) => (
                <article key={`${screen.id ?? "log"}-${index}`} className="log-item">
                  <span className="pill">Page {index + 1}</span>
                  <p>{(screen.lines ?? []).join(" ")}</p>
                </article>
              ))}
            </div>
          </aside>
        </div>
      ) : null}
    </div>
  );
}

function ReviewView({
  course,
  practice,
  reviewHistory,
  onRate
}: {
  course: CourseRecord;
  practice: PracticeState;
  reviewHistory: Record<string, ReviewHistoryEntry>;
  onRate: (questionId: string, rating: number) => void;
}) {
  const [index, setIndex] = React.useState(0);
  const [flipped, setFlipped] = React.useState(false);

  React.useEffect(() => {
    setIndex(0);
    setFlipped(false);
  }, [course.courseId]);

  if (practice.loading) {
    return <div className="loading-state">Building recall queue...</div>;
  }
  if (practice.error || !practice.data) {
    return <div className="empty-state">{practice.error ?? "No practice assets found."}</div>;
  }

  const queue = [...practice.data.flashcards].sort((left, right) => {
    const leftHistory = left.variant.question_id ? reviewHistory[left.variant.question_id] : undefined;
    const rightHistory = right.variant.question_id ? reviewHistory[right.variant.question_id] : undefined;
    const leftScore = (leftHistory?.lastRating ?? 0) * 100 + (leftHistory?.seenCount ?? 0);
    const rightScore = (rightHistory?.lastRating ?? 0) * 100 + (rightHistory?.seenCount ?? 0);
    return leftScore - rightScore;
  });

  if (!queue.length) {
    return <div className="empty-state">This course has no flashcards yet.</div>;
  }

  const current = queue[index % queue.length];
  const prompt =
    current.variant.front ??
    current.variant.stem ??
    current.variant.prompt_blocks?.join(" / ") ??
    current.family.learning_goal ??
    "Recall prompt";
  const answer =
    current.variant.back ?? current.variant.reference_answer?.join(" ") ?? current.variant.explanation ?? "No answer available.";

  return (
    <section className="review-shell">
      <div className="section-heading">
        <div>
          <h2>Recall Session</h2>
          <p>{course.title} · FSRS-flavored review loop using generated flashcard families.</p>
        </div>
      </div>

      <div className="review-summary-grid">
        <div className="metric-card">
          <strong>{queue.length}</strong>
          <p>Total flashcards in the current course.</p>
        </div>
        <div className="metric-card">
          <strong>{queue.filter((item) => !item.variant.question_id || !reviewHistory[item.variant.question_id]).length}</strong>
          <p>Cards you have not seen yet.</p>
        </div>
        <div className="metric-card">
          <strong>{queue.filter((item) => {
            const id = item.variant.question_id;
            return id ? (reviewHistory[id]?.lastRating ?? 0) < 4 : true;
          }).length}</strong>
          <p>Cards still worth refreshing.</p>
        </div>
      </div>

      <div className="review-card-flip" onClick={() => setFlipped((value) => !value)}>
        <article className="review-card-shell">
          <div className={`review-card-body ${flipped ? "flipped" : ""}`}>
            <div className="review-face front">
              <div className="review-card-meta">
                <span className="pill accent">{current.chapterTitle}</span>
                <span className="pill">{current.stepId}</span>
              </div>
              <h3>{prompt}</h3>
              <p className="story-note">Tap the card to reveal the target answer.</p>
            </div>

            <div className="review-face back">
              <div className="review-card-meta">
                <span className="pill accent">Definition</span>
                <span className="pill">{current.family.difficulty ?? "review"}</span>
              </div>
              <h3>{prompt}</h3>
              <div className="review-card-answer">{answer}</div>
            </div>
          </div>
        </article>
      </div>

      {flipped ? (
        <div className="card-rating-row">
          {[
            ["Again", 1],
            ["Hard", 2],
            ["Good", 3],
            ["Easy", 4]
          ].map(([label, rating]) => (
            <button
              key={label}
              className="rating-button"
              onClick={() => {
                if (current.variant.question_id) {
                  onRate(current.variant.question_id, Number(rating));
                }
                setFlipped(false);
                setIndex((value) => value + 1);
              }}
            >
              <strong>{label}</strong>
              <div className="muted">Rate the recall quality and move on.</div>
            </button>
          ))}
        </div>
      ) : null}
    </section>
  );
}

function VaultView({ course, practice }: { course: CourseRecord; practice: PracticeState }) {
  const [filter, setFilter] = React.useState<"all" | "flashcards" | "quizzes" | "longform">("all");

  if (practice.loading) {
    return <div className="loading-state">Opening the vault...</div>;
  }
  if (practice.error || !practice.data) {
    return <div className="empty-state">{practice.error ?? "No vault assets found."}</div>;
  }

  const entries = [
    ...practice.data.flashcards.map((item) => ({
      key: item.variant.question_id ?? `${item.stepId}-${item.family.family_id ?? "flash"}`,
      type: "flashcards" as const,
      title: item.variant.front ?? item.family.learning_goal ?? "Flashcard",
      body: item.variant.back ?? item.variant.explanation ?? item.family.retrieval_focus ?? "Recall card",
      meta: item.chapterTitle
    })),
    ...practice.data.quizFamilies.map((family) => ({
      key: family.family_id ?? family.learning_goal ?? "quiz",
      type: "quizzes" as const,
      title: family.learning_goal ?? family.family_id ?? "Quiz family",
      body: family.variants?.[0]?.stem ?? family.variants?.[0]?.front ?? family.retrieval_focus ?? "Quiz family",
      meta: family.difficulty ?? "quiz"
    })),
    ...practice.data.longformFamilies.map((family) => ({
      key: family.family_id ?? family.learning_goal ?? "longform",
      type: "longform" as const,
      title: family.learning_goal ?? family.family_id ?? "Longform family",
      body: family.variants?.[0]?.stem ?? family.retrieval_focus ?? "Longform prompt",
      meta: family.difficulty ?? "longform"
    }))
  ].filter((entry) => filter === "all" || entry.type === filter);

  return (
    <section>
      <div className="section-heading">
        <div>
          <h2>The Vault</h2>
          <p>{course.title} · generated practice assets organized into a playable challenge bank.</p>
        </div>
      </div>

      <div className="bank-filters">
        {[
          ["all", "Everything"],
          ["flashcards", "Flashcards"],
          ["quizzes", "Quizzes"],
          ["longform", "Longform"]
        ].map(([value, label]) => (
          <button
            key={value}
            className={`pill-button ${filter === value ? "primary" : ""}`}
            onClick={() => setFilter(value as typeof filter)}
          >
            {label}
          </button>
        ))}
      </div>

      <div className="vault-grid">
        {entries.map((entry) => (
          <article key={entry.key} className="vault-card">
            <div className="vault-meta">
              <span className="pill accent">{entry.type}</span>
              <span className="pill">{entry.meta}</span>
            </div>
            <h4>{entry.title}</h4>
            <p>{entry.body}</p>
          </article>
        ))}
      </div>
    </section>
  );
}

function TextbookView({
  chapter,
  textbook,
  practice
}: {
  chapter: ChapterRecord;
  textbook: TextbookState;
  practice: QuestionIndex | null;
}) {
  const compiled = useCompiledMdx(textbook.data);

  const mdxComponents = React.useMemo<Record<string, React.ComponentType<any>>>(() => {
    const QuestionRef = ({ id }: { id: string }) => {
      const entry = practice?.questions.get(id);
      const prompt =
        entry?.variant.front ??
        entry?.variant.stem ??
        entry?.family.learning_goal ??
        "Practice prompt";
      const answer = entry?.variant.back ?? entry?.variant.reference_answer?.join(" ") ?? entry?.variant.explanation;
      return (
        <aside className="inline-practice">
          <h4>Quick Practice</h4>
          <p>{prompt}</p>
          {answer ? <p className="muted">{answer}</p> : null}
        </aside>
      );
    };

    const QuestionFamily = ({ familyId }: { familyId: string }) => {
      const family = practice?.families.get(familyId);
      return (
        <article className="inline-practice">
          <h4>{family?.learning_goal ?? familyId}</h4>
          <p>{family?.retrieval_focus ?? family?.variants?.[0]?.front ?? "Practice family"}</p>
        </article>
      );
    };

    return {
      KeyPoint: ({ children }: { children: React.ReactNode }) => (
        <aside className="inline-practice">
          <h4>Key Point</h4>
          <p>{flattenText(children)}</p>
        </aside>
      ),
      Definition: ({ title, children }: { title: string; children: React.ReactNode }) => (
        <section className="inline-practice">
          <h4>{title}</h4>
          <p>{flattenText(children)}</p>
        </section>
      ),
      Example: ({ title, children }: { title: string; children: React.ReactNode }) => (
        <section className="inline-practice">
          <h4>{title}</h4>
          <div>{children}</div>
        </section>
      ),
      QuestionRef,
      QuestionFamily
    };
  }, [practice]);

  return (
    <section className="textbook-shell">
      <div className="section-heading">
        <div>
          <h2>{chapter.title}</h2>
          <p>Textbook mode keeps the content readable while embedding the generated practice back into the reading flow.</p>
        </div>
      </div>

      {textbook.loading || compiled.loading ? <div className="loading-state">Preparing the reading room...</div> : null}
      {textbook.error ? <div className="empty-state">{textbook.error}</div> : null}
      {compiled.error ? <div className="empty-state">{compiled.error}</div> : null}
      {compiled.data ? (
        <div className="mdx-shell">
          {React.createElement(compiled.data, { components: mdxComponents })}
        </div>
      ) : null}
    </section>
  );
}

function TermDrawer({
  selectedTerm,
  onClose
}: {
  selectedTerm: { id: string; term: TermEntry };
  onClose: () => void;
}) {
  return (
    <div className="scrim" onClick={onClose}>
      <aside className="term-panel" onClick={(event) => event.stopPropagation()}>
        <div className="drawer-head">
          <h3>{selectedTerm.term.display ?? selectedTerm.id}</h3>
          <button className="pill-button ghost" onClick={onClose}>
            Close
          </button>
        </div>
        <div className="term-panel-copy">
          <span className="pill accent">Glossary Entry</span>
          <p>{selectedTerm.term.gloss ?? "No glossary note available yet."}</p>
          {selectedTerm.term.aliases?.length ? (
            <>
              <strong>Aliases</strong>
              <div className="term-chip-row">
                {selectedTerm.term.aliases.map((alias) => (
                  <span key={alias} className="pill">{alias}</span>
                ))}
              </div>
            </>
          ) : null}
        </div>
      </aside>
    </div>
  );
}

function LearningDock({
  course,
  chapter,
  dueCards,
  onHome,
  onContinue,
  onReview,
  onVault
}: {
  course: CourseRecord;
  chapter: ChapterRecord | null;
  dueCards: number;
  onHome: () => void;
  onContinue: () => void;
  onReview: () => void;
  onVault: () => void;
}) {
  return (
    <div className="learning-dock">
      <div className="dock-copy">
        <span className="eyebrow">Learning Dock</span>
        <strong>{course.title}</strong>
        <span>{chapter ? `${chapter.lessonId} · ${chapter.title}` : "Choose a route"}</span>
      </div>
      <div className="dock-actions">
        <button className="pill-button ghost" onClick={onHome}>
          Home
        </button>
        <button className="pill-button primary" onClick={onContinue} disabled={!chapter}>
          Continue
        </button>
        <button className="pill-button ghost" onClick={onReview}>
          Recall
        </button>
        <button className="pill-button ghost" onClick={onVault}>
          Vault
        </button>
        <span className="pill accent">{dueCards} due</span>
      </div>
    </div>
  );
}

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
