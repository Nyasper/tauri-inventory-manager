class FeedbackService {
  message = $state("");
  isError = $state(false);
  private timer: ReturnType<typeof setTimeout> | undefined;
  success(message: string) {
    this.show(message, false);
  }
  error(message: string) {
    this.show(message, true);
  }
  private show(message: string, isError: boolean) {
    clearTimeout(this.timer);
    this.message = message;
    this.isError = isError;
    if (!isError) this.timer = setTimeout(() => (this.message = ""), 4000);
  }
}
export const feedbackService = new FeedbackService();
