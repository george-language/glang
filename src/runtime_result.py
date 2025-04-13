class RuntimeResult:
    def __init__(self):
        self.reset()

    def reset(self):
        self.value = None
        self.error = None
        self.func_return_value = None
        self.loop_should_continue = False
        self.loop_should_break = False

    def register(self, result):
        self.error = result.error
        self.func_return_value = result.func_return_value
        self.loop_should_continue = result.loop_should_continue
        self.loop_should_break = result.loop_should_break

        return result.value

    def success(self, value):
        self.reset()
        self.value = value

        return self

    def successReturn(self, value):
        self.reset()
        self.func_return_value = value

        return self

    def successContinue(self):
        self.reset()
        self.loop_should_continue = True

        return self

    def successBreak(self):
        self.reset()
        self.loop_should_break = True

        return self

    def failure(self, error):
        self.reset()
        self.error = error

        return self

    def shouldReturn(self):
        return (
            self.error or
            self.func_return_value or
            self.loop_should_continue or
            self.loop_should_break
        )
