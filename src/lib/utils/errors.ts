/** Error de un comando del backend con su código estable. */
export class AppError extends Error {
  readonly code: string;

  constructor(code: string, message: string) {
    super(message);
    this.name = "AppError";
    this.code = code;
  }
}

interface BackendError {
  code?: unknown;
  message?: unknown;
}

/** Convierte cualquier rechazo (objeto del backend, Error o string) en un `AppError`. */
export function toAppError(cause: unknown): AppError {
  if (cause instanceof AppError) return cause;
  if (cause && typeof cause === "object") {
    const { code, message } = cause as BackendError;
    if (typeof message === "string") {
      return new AppError(typeof code === "string" ? code : "unknown", message);
    }
  }
  return new AppError("unknown", typeof cause === "string" ? cause : String(cause));
}

/** Mensaje legible de un error de comando. */
export function errorMessage(cause: unknown): string {
  return toAppError(cause).message;
}

/** Indica si el error corresponde a un nombre duplicado. */
export function isDuplicateError(cause: unknown): boolean {
  return toAppError(cause).code.startsWith("duplicate");
}
