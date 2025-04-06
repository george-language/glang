from src.errors import RunTimeError


class Number:
    def __init__(self, value):
        self.value = value

        self.setPos()
        self.setContext()

    def setPos(self, pos_start=None, pos_end=None):
        self.pos_start = pos_start
        self.pos_end = pos_end

        return self

    def setContext(self, context=None):
        self.context = context
        return self

    def addedTo(self, other):
        if isinstance(other, Number):
            return Number(self.value + other.value).setContext(self.context), None

    def subtractedBy(self, other):
        if isinstance(other, Number):
            return Number(self.value - other.value).setContext(self.context), None

    def multipliedBy(self, other):
        if isinstance(other, Number):
            return Number(self.value * other.value).setContext(self.context), None

    def dividedBy(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                return None, RunTimeError(
                    other.pos_start, other.pos_end, 'Division by zero',
                    self.context
                )

            return Number(self.value / other.value).setContext(self.context), None

    def poweredBy(self, other):
        if isinstance(other, Number):
            return Number(self.value ** other.value).setContext(self.context), None

    def copy(self):
        copy = Number(self.value)
        copy.setPos(self.pos_start, self.pos_end)
        copy.setContext(self.context)
        return copy

    def __repr__(self):
        return f'{self.value}'
