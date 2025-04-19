from src.strings_with_arrows import string_with_arrows


class Error:
    def __init__(self, pos_start, pos_end, error_name: str, details: str):
        self.pos_start = pos_start
        self.pos_end = pos_end
        self.error_name = error_name
        self.details = details

    def asString(self):
        result = f'{self.error_name}: {self.details}'
        result += f'\nFile "{self.pos_start.file_name}", Line {self.pos_start.line_num + 1}'
        result += f'\n\n{string_with_arrows(self.pos_start.file_text, self.pos_start, self.pos_end)}'
        return result


class IllegalCharError(Error):
    def __init__(self, pos_start, pos_end, details: str):
        super().__init__(pos_start, pos_end, 'Illegal Character Error', details)


class InvalidSyntaxError(Error):
    def __init__(self, pos_start, pos_end, details: str):
        super().__init__(pos_start, pos_end, 'Illegal Syntax Error', details)


class RunTimeError(Error):
    ColorError = '\033[91m'
    ColorEnd = '\033[0m'

    def __init__(self, pos_start, pos_end, details: str, context):
        super().__init__(pos_start, pos_end, 'Runtime Error', details)
        self.context = context

    def asString(self):
        result = self.generateTraceback()
        result += f'{self.error_name}: {self.details}'
        result += f'\n\n{string_with_arrows(self.pos_start.file_text, self.pos_start, self.pos_end)}' + RunTimeError.ColorEnd
        return result

    def generateTraceback(self):
        result = ''
        pos = self.pos_start
        ctx = self.context

        while ctx:
            result = f'   File {pos.file_name}, Line {str(pos.line_num + 1)}, In {ctx.display_name}\n' + result
            pos = ctx.parent_entry_pos
            ctx = ctx.parent

        return RunTimeError.ColorError + 'Traceback (most recent call last):\n' + result


class ExpectedCharacterError(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, 'Expected Character', details)
