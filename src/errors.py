class Error:
    def __init__(self, error_name: str, details: str):
        self.error_name = error_name
        self.details = details

    def asString(self):
        return f'{self.error_name}: {self.details}'

class IllegalCharError(Error):
    def __init__(self, details: str):
        super().__init__('Illegal Character Error', details)


class WrongFileTypeError(Error):
    def __init__(self, details: str):
        super().__init__('Wrong File Type', details)