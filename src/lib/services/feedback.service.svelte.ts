class Feedback {
  message = $state("");
  isError = $state(false);
  private timer: ReturnType<typeof setTimeout> | undefined;
  success(message: string) {
    this.show(message, false);
  }
  error(message: string) {
    this.show(message, true);
  }
  private show(message: string, error: boolean) {
    clearTimeout(this.timer);
    this.message = message;
    this.isError = error;
    if (!error) this.timer = setTimeout(() => (this.message = ""), 4000);
  }
}
export const feedback = new Feedback();
