import { AlertTriangle, CheckCircle2, Sparkles, Trophy } from "lucide-react";
import type { CourseRecord, CourseSummary } from "../player-model";
import { courseUnavailableMessage, isCourseReady, safeColor } from "../player-utils";
import "./LibraryView.css";

export function LibraryView({
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
          const isBlocked = !isCourseReady(course);
          const color = safeColor(course.brandColor);
          return (
            <article
              key={course.courseId}
              className={`course-card ${isBlocked ? "disabled" : ""}`}
              style={{ ["--card-accent" as string]: `${color}22` }}
              aria-disabled={isBlocked}
              onClick={() => {
                if (!isBlocked) {
                  onOpenCourse(course.courseId);
                }
              }}
            >
              <div className="course-card-head">
                <div className="metric-row">
                  <span className="pill">
                    <Sparkles size={13} />
                    <span>{course.category ?? "Course"}</span>
                  </span>
                  <span className={`pill ${isBlocked ? "danger" : "accent"}`}>
                    {isBlocked ? <AlertTriangle size={13} /> : <CheckCircle2 size={13} />}
                    <span>{isBlocked ? "Blocked" : "Ready"}</span>
                  </span>
                </div>
                <div className="course-emblem" style={{ background: `linear-gradient(135deg, ${color}, #60a5fa)` }}>
                  {course.courseId.slice(0, 2).toUpperCase()}
                </div>
                <h3>{course.title}</h3>
                <p>{course.chapters.length} chapters · {stats?.totalSteps ?? 0} scenes · progression tracked locally</p>
                {isBlocked ? (
                  <div className="validation-error">
                    {(course.validationErrors ?? [{ code: "BLOCKED", message: courseUnavailableMessage(course) }]).slice(0, 3).map((error, index) => (
                      <span key={`${error.code ?? "validation"}-${index}`}>
                        {error.path ? `${error.path}: ` : ""}
                        {error.message}
                      </span>
                    ))}
                  </div>
                ) : null}
              </div>
              <div className="course-card-footer">
                <div className="metric-row">
                  <span className="pill accent">
                    <Trophy size={13} />
                    <span>{stats?.mastery ?? 0}% mastery</span>
                  </span>
                  <span className="pill">
                    <CheckCircle2 size={13} />
                    <span>{stats?.completedSteps ?? 0} scenes cleared</span>
                  </span>
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
