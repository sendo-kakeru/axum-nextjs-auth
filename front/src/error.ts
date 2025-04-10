export const problemDetailsType = {
  VALIDATE: "https://example.com/problems/validate",
  DUPLICATE: "https://example.com/problems/duplicate",
  NOT_FOUND: "https://example.com/problems/not-found",
  NO_CONTENT: "https://example.com/problems/no-content",
  INVALID_JSON: "https://example.com/problems/invalid-json",
  METHOD_NOT_ALLOWED: "https://example.com/problems/method-not-allowed",
  BAD_REQUEST: "https://example.com/problems/bad-request",
  UNSUPPORTED_MEDIA_TYPE: "https://example.com/problems/unsupported-media-type",
  CONFLICT: "https://example.com/problems/conflict",
  UNAUTHORIZED: "https://example.com/problems/unauthorized",
  FORBIDDEN: "https://example.com/problems/forbidden",
  INTERNAL_SERVER_ERROR: "https://example.com/problems/internal-server-error",
} as const;

export type ProblemDetails = {
  type?: (typeof problemDetailsType)[keyof typeof problemDetailsType];
  title?: string;
  instance?: string;
  detail?: string;
};

export class ProblemDetailsError extends Error {
  problemDetails: ProblemDetails;

  constructor(problemDetails: ProblemDetails) {
    super(problemDetails.title ?? "問題が発生しました");
    this.problemDetails = problemDetails;
  }

  static async throwProblemDetails(response: Response) {
    if (
      response.headers.get("Content-Type")?.includes("application/problem+json")
    ) {
      const json: ProblemDetails | undefined = await response.json();
      if (!json) throw new Error("予期しないエラーが発生しました");

      throw new ProblemDetailsError(json);
    }
  }
}

// createErrorMessageを作ってもいいかも。
export type Data<T> =
  | {
      status: "success";
      data: T;
    }
  | {
      status: "error";
      message: string;
    };

