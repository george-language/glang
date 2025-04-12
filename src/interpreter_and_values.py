from src.token_types_and_keywords import *
from src.context import Context
from src.runtime_result import RuntimeResult
from src.symbol_table import SymbolTable
from src.nodes import NumberNode, VariableAccessNode, VariableAssignNode, BinaryOperatorNode, UnaryOperatorNode, \
    StringNode, ListNode
from src.errors import RunTimeError


class Interpreter:
    def visit(self, node, context):
        method_name = f'visit_{type(node).__name__}'
        method = getattr(self, method_name, self.noVisitMethod)

        return method(node, context)

    def noVisitMethod(self, node, context):
        raise Exception(f'No visit_{type(node).__name__} method defined')

    def visit_NumberNode(self, node: NumberNode, context):
        return RuntimeResult().success(
            Number(node.token.value).setContext(context).setPos(node.pos_start, node.pos_end))

    def visit_StringNode(self, node: StringNode, context):
        return RuntimeResult().success(
            String(node.token.value).setContext(context).setPos(node.pos_start, node.pos_end))

    def visit_ListNode(self, node: ListNode, context):
        result = RuntimeResult()
        elements = []

        for element in node.element_nodes:
            elements.append(result.register(self.visit(element, context)))

            if result.error:
                return result

        return result.success(List(elements).setContext(context).setPos(node.pos_start, node.pos_end))

    def visit_VariableAccessNode(self, node: VariableAccessNode, context):
        result = RuntimeResult()
        var_name = node.var_name_token.value
        value = context.symbol_table.get(var_name)

        if not value:
            return result.failure(
                RunTimeError(node.pos_start,
                             node.pos_end,
                             f'"{var_name}" is not defined',
                             context))

        value = value.copy().setPos(node.pos_start, node.pos_end).setContext(context)
        return result.success(value)

    def visit_VariableAssignNode(self, node: VariableAssignNode, context):
        result = RuntimeResult()
        var_name = node.var_name_token.value
        value = result.register(self.visit(node.value_node, context))

        if result.error:
            return result

        context.symbol_table.set(var_name, value)
        return result.success(value)

    def visit_BinaryOperatorNode(self, node: BinaryOperatorNode, context):
        result = RuntimeResult()
        left = result.register(self.visit(node.left_node, context))

        if result.error:
            return result

        right = result.register(self.visit(node.right_node, context))

        if result.error:
            return result

        if node.op_token.type == TT_PLUS:
            number, error = left.addedTo(right)
        elif node.op_token.type == TT_MINUS:
            number, error = left.subtractedBy(right)
        elif node.op_token.type == TT_MUL:
            number, error = left.multipliedBy(right)
        elif node.op_token.type == TT_DIV:
            number, error = left.dividedBy(right)
        elif node.op_token.type == TT_POW:
            number, error = left.poweredBy(right)
        elif node.op_token.type == TT_EE:
            number, error = left.getComparisonEq(right)
        elif node.op_token.type == TT_NE:
            number, error = left.getComparisonNe(right)
        elif node.op_token.type == TT_LT:
            number, error = left.getComparisonLt(right)
        elif node.op_token.type == TT_GT:
            number, error = left.getComparisonGt(right)
        elif node.op_token.type == TT_LTE:
            number, error = left.getComparisonLte(right)
        elif node.op_token.type == TT_GTE:
            number, error = left.getComparisonGte(right)
        elif node.op_token.matches(TT_KEYWORD, 'and'):
            number, error = left.andedBy(right)
        elif node.op_token.matches(TT_KEYWORD, 'or'):
            number, error = left.oredBy(right)

        if error:
            return result.failure(error)

        else:
            return result.success(number.setPos(node.pos_start, node.pos_end))

    def visit_UnaryOperatorNode(self, node: UnaryOperatorNode, context):
        result = RuntimeResult()
        number = result.register(self.visit(node.node, context))

        if result.error:
            return result

        error = None

        if node.op_token.type == TT_MINUS:
            number, error = number.multipliedBy(Number(-1))

        elif node.op_token.matches(TT_KEYWORD, 'oppositeof'):
            number, error = number.notted()

        if error:
            return result.failure(error)

        else:
            return result.success(number.setPos(node.pos_start, node.pos_end))

    def visit_IfNode(self, node, context):
        result = RuntimeResult()

        for condition, expr in node.cases:
            condition_value = result.register(self.visit(condition, context))

            if result.error:
                return result

            if condition_value.isTrue():
                expr_value = result.register(self.visit(expr, context))

                if result.error:
                    return result

                return result.success(expr_value)

        if node.else_case:
            else_value = result.register(self.visit(node.else_case, context))

            if result.error:
                return result

            return result.success(else_value)

        return result.success(None)

    def visit_ForNode(self, node, context):
        result = RuntimeResult()
        elements = []

        start_value = result.register(self.visit(node.start_value_node, context))

        if result.error:
            return result

        end_value = result.register(self.visit(node.end_value_node, context))

        if result.error:
            return result

        if node.step_value_node:
            step_value = result.register(self.visit(node.step_value_node, context))

            if result.error:
                return result

        else:
            step_value = Number(1)

        i = start_value.value

        if step_value.value >= 0:
            condition = lambda: i < end_value.value

        else:
            condition = lambda: i > end_value.value

        while condition():
            context.symbol_table.set(node.var_name_token.value, Number(i))
            i += step_value.value

            elements.append(result.register(self.visit(node.body_node, context)))

            if result.error:
                return result

        return result.success(List(elements).setContext(context).setPos(node.pos_start, node.pos_end))

    def visit_WhileNode(self, node, context):
        result = RuntimeResult()
        elements = []

        while True:
            condition = result.register(self.visit(node.condition_node, context))

            if result.error:
                return result

            if not condition.isTrue():
                break

            elements.append(result.register(self.visit(node.body_node, context)))

            if result.error:
                return result

        return result.success(List(elements).setContext(context).setPos(node.pos_start, node.pos_end))

    def visit_FunctionDefinitionNode(self, node, context):
        result = RuntimeResult()

        func_name = node.var_name_token.value if node.var_name_token else None
        body_node = node.body_node
        arg_names = [arg_name.value for arg_name in node.arg_name_tokens]
        func_value = Function(func_name, body_node, arg_names).setContext(context).setPos(node.pos_start,
                                                                                          node.pos_end)

        if node.var_name_token:
            context.symbol_table.set(func_name, func_value)

        return result.success(func_value)

    def visit_CallNode(self, node, context):
        result = RuntimeResult()
        args = []

        value_to_call = result.register(self.visit(node.node_to_call, context))

        if result.error:
            return result

        value_to_call = value_to_call.copy().setPos(node.pos_start, node.pos_end)

        for arg_node in node.arg_nodes:
            args.append(result.register(self.visit(arg_node, context)))

            if result.error:
                return result

        return_value = result.register(value_to_call.execute(args))

        if result.error:
            return result

        return_value = return_value.copy().setPos(node.pos_start, node.pos_end).setContext(context)

        return result.success(return_value)


class Value:
    def __init__(self):
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
        return None, self.illegalOperation(other)

    def subtractedBy(self, other):
        return None, self.illegalOperation(other)

    def multipliedBy(self, other):
        return None, self.illegalOperation(other)

    def dividedBy(self, other):
        return None, self.illegalOperation(other)

    def poweredBy(self, other):
        return None, self.illegalOperation(other)

    def getComparisonEq(self, other):
        return None, self.illegalOperation(other)

    def getComparisonNe(self, other):
        return None, self.illegalOperation(other)

    def getComparisonLt(self, other):
        return None, self.illegalOperation(other)

    def getComparisonGt(self, other):
        return None, self.illegalOperation(other)

    def getComparisonLte(self, other):
        return None, self.illegalOperation(other)

    def getComparisonGte(self, other):
        return None, self.illegalOperation(other)

    def andedBy(self, other):
        return None, self.illegalOperation(other)

    def oredBy(self, other):
        return None, self.illegalOperation(other)

    def notted(self):
        return None, self.illegalOperation()

    def copy(self):
        raise Exception('No copy method defined')

    def isTrue(self):
        return False

    def execute(self, args):
        return RuntimeResult().failure(self.illegalOperation())

    def illegalOperation(self, other=None):
        if not other:
            other = self

        return RunTimeError(
            self.pos_start, other.pos_end,
            'Illegal operation',
            self.context
        )


class Number(Value):
    def __init__(self, value):
        super().__init__()
        self.value = value

    def addedTo(self, other):
        if isinstance(other, Number):
            return Number(self.value + other.value).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def subtractedBy(self, other):
        if isinstance(other, Number):
            return Number(self.value - other.value).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def multipliedBy(self, other):
        if isinstance(other, Number):
            return Number(self.value * other.value).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def dividedBy(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                return None, RunTimeError(
                    other.pos_start, other.pos_end, 'Division by zero',
                    self.context
                )

            return Number(self.value / other.value).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def poweredBy(self, other):
        if isinstance(other, Number):
            return Number(self.value ** other.value).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def getComparisonEq(self, other):
        if isinstance(other, Number):
            return Number(int(self.value == other.value)).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def getComparisonNe(self, other):
        if isinstance(other, Number):
            return Number(int(self.value != other.value)).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def getComparisonLt(self, other):
        if isinstance(other, Number):
            return Number(int(self.value < other.value)).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def getComparisonGt(self, other):
        if isinstance(other, Number):
            return Number(int(self.value > other.value)).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def getComparisonLte(self, other):
        if isinstance(other, Number):
            return Number(int(self.value <= other.value)).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def getComparisonGte(self, other):
        if isinstance(other, Number):
            return Number(int(self.value >= other.value)).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def andedBy(self, other):
        if isinstance(other, Number):
            return Number(int(self.value and other.value)).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def oredBy(self, other):
        if isinstance(other, Number):
            return Number(int(self.value or other.value)).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def notted(self):
        return Number(1 if self.value == 0 else 0).setContext(self.context), None

    def copy(self):
        copy = Number(self.value)
        copy.setPos(self.pos_start, self.pos_end)
        copy.setContext(self.context)
        return copy

    def isTrue(self):
        return self.value != 0

    def __str__(self):
        return f'{self.value}'

    def __repr__(self):
        return f'<number: {self.value}>'


class String(Value):
    def __init__(self, value):
        super().__init__()
        self.value = value

    def addedTo(self, other):
        if isinstance(other, String):
            return String(self.value + other.value).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def multipliedBy(self, other):
        if isinstance(other, Number):
            return String(self.value * other.value).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def subtractedBy(self, other):
        if isinstance(other, String):
            return String(other.value).setContext(self.context), None

        else:
            return None, Value.illegalOperation(self, other)

    def isTrue(self):
        return len(self.value) > 0

    def copy(self):
        copy = String(self.value)
        copy.setPos(self.pos_start, self.pos_end)
        copy.setContext(self.context)
        return copy

    def __str__(self):
        return self.value

    def __repr__(self):
        return f'<string: {self.value}>'


class List(Value):
    def __init__(self, elements):
        super().__init__()
        self.elements = elements

    def addedTo(self, other):
        if isinstance(other, List):
            new_list = self.copy()
            new_list.elements.extend(other.elements)

            return new_list, None

        else:
            None, Value.illegalOperation(self, other)

    def subtractedBy(self, other):
        if isinstance(other, Number):
            new_list = self.copy()

            try:
                new_list.elements.pop(other.value)

                return new_list, None

            except:
                return None, RunTimeError(
                    other.pos_start, other.pos_end,
                    'List index is out of bounds',
                    self.context
                )

        else:
            None, Value.illegalOperation(self, other)

    def multipliedBy(self, other):
        new_list = self.copy()
        new_list.elements.append(other)

        return new_list, None

    def poweredBy(self, other):
        if isinstance(other, Number):
            try:
                return self.elements[other.value], None

            except:
                return None, RunTimeError(
                    other.pos_start, other.pos_end,
                    'List index is out of bounds',
                    self.context
                )
        else:
            return None, Value.illegalOperation(self, other)

    def copy(self):
        copy = List(self.elements)
        copy.setPos(self.pos_start, self.pos_end)
        copy.setContext(self.context)

        return copy

    def __str__(self):
        return f'{", ".join([str(x) for x in self.elements])}'

    def __repr__(self):
        return f'<list: [{", ".join([str(x) for x in self.elements])}]>'


class BaseFunction(Value):
    def __init__(self, name):
        super().__init__()
        self.name = name or '<anonymous>'

    def generateNewContext(self):
        new_context = Context(self.name, self.context, self.pos_start)
        new_context.symbol_table = SymbolTable(new_context.parent.symbol_table)

        return new_context

    def checkArgs(self, arg_names, args):
        result = RuntimeResult()

        if len(args) > len(arg_names) or len(args) < len(arg_names):
            return result.failure(RunTimeError(
                self.pos_start, self.pos_end,
                f'{self.name} takes {len(arg_names)} positional argument(s) but the program gave {len(args)}',
                self.context
            ))

        return result.success(None)

    def populateArgs(self, arg_names, args, exec_ctx):
        for i in range(len(args)):
            arg_name = arg_names[i]
            arg_value = args[i]
            arg_value.setContext(exec_ctx)
            exec_ctx.symbol_table.set(arg_name, arg_value)

    def checkAndPopulateArgs(self, arg_names, args, exec_ctx):
        result = RuntimeResult()
        result.register(self.checkArgs(arg_names, args))

        if result.error:
            return result

        self.populateArgs(arg_names, args, exec_ctx)
        return result.success(None)


class Function(BaseFunction):
    def __init__(self, name, body_node, arg_names):
        super().__init__(name)
        self.body_node = body_node
        self.arg_names = arg_names

    def execute(self, args):
        result = RuntimeResult()
        interpreter = Interpreter()
        exec_context = self.generateNewContext()

        result.register(self.checkAndPopulateArgs(self.arg_names, args, exec_context))

        if result.error:
            return result

        value = result.register(interpreter.visit(self.body_node, exec_context))

        if result.error:
            return result

        return result.success(value)

    def copy(self):
        copy = Function(self.name, self.body_node, self.arg_names)
        copy.setPos(self.pos_start, self.pos_end)
        copy.setContext(self.context)
        return copy

    def __repr__(self):
        return f'<function: {self.name}>'


class BuiltInFunction(BaseFunction):
    def __init__(self, name):
        super().__init__(name)

    def execute(self, args):
        result = RuntimeResult()
        exec_ctx = self.generateNewContext()

        method_name = f'execute_{self.name}'
        method = getattr(self, method_name, self.noVisitMethod)

        result.register(self.checkAndPopulateArgs(method.arg_names, args, exec_ctx))

        if result.error:
            return result

        return_value = result.register(method(exec_ctx))

        if result.error:
            return result

        return result.success(return_value)

    def noVisitMethod(self, node, context):
        raise Exception(f'No execute_{self.name} method defined')

    def copy(self):
        copy = BuiltInFunction(self.name)
        copy.setPos(self.pos_start, self.pos_end)
        copy.setContext(self.context)

        return copy

    def __repr__(self):
        return f'<built-in function: {self.name}>'

    # --------- Built In Functions --------- #

    def execute_print(self, exec_ctx):
        print(str(exec_ctx.symbol_table.get('value')))

        return RuntimeResult().success(Number.null)

    def execute_input(self, exec_ctx):
        text = input()

        return RuntimeResult().success(String(text))

    def execute_isnumber(self, exec_ctx):
        is_instance = isinstance(exec_ctx.symbol_table.get('value'), Number)

        return RuntimeResult().success(Number.true if is_instance else Number.false)

    def execute_isstring(self, exec_ctx):
        is_instance = isinstance(exec_ctx.symbol_table.get('value'), String)

        return RuntimeResult().success(Number.true if is_instance else Number.false)

    def execute_islist(self, exec_ctx):
        is_instance = isinstance(exec_ctx.symbol_table.get('value'), List)

        return RuntimeResult().success(Number.true if is_instance else Number.false)

    def execute_isbasefunction(self, exec_ctx):
        is_instance = isinstance(exec_ctx.symbol_table.get('value'), BaseFunction)

        return RuntimeResult().success(Number.true if is_instance else Number.false)

    def execute_append(self, exec_ctx):
        list_obj = exec_ctx.symbol_table.get('list')
        value = exec_ctx.symbol_table.get('value')

        if not isinstance(list_obj, List):
            return RuntimeResult().failure(RunTimeError(
                self.pos_start, self.pos_end, 'First argument is not type list', exec_ctx
            ))

        list_obj.elements.append(value)
        return RuntimeResult().success(Number.null)

    def execute_pop(self, exec_ctx):
        list_obj = exec_ctx.symbol_table.get('list')
        value = exec_ctx.symbol_table.get('value')

        if not isinstance(list_obj, List):
            return RuntimeResult().failure(RunTimeError(
                self.pos_start, self.pos_end, 'First argument is not type list', exec_ctx
            ))

        if not isinstance(value, Number):
            return RuntimeResult().failure(RunTimeError(
                self.pos_start, self.pos_end, 'Second argument is not type number', exec_ctx
            ))

        try:
            element = list_obj.elements.pop(value.value)

        except:
            return RuntimeResult().failure(RunTimeError(
                self.pos_start, self.pos_end, 'List index out of range', exec_ctx
            ))

        return RuntimeResult().success(element)

    def execute_extend(self, exec_ctx):
        list_a = exec_ctx.symbol_table.get('list_a')
        list_b = exec_ctx.symbol_table.get('list_b')

        if not isinstance(list_a, List):
            return RuntimeResult().failure(RunTimeError(
                self.pos_start, self.pos_end, 'First argument is not type list', exec_ctx
            ))

        if not isinstance(list_b, List):
            return RuntimeResult().failure(RunTimeError(
                self.pos_start, self.pos_end, 'Second argument is not type list', exec_ctx
            ))

        list_a.elements.append(list_b)
        return RuntimeResult().success(Number.null)

    def execute_reverse(self, exec_ctx):
        list_obj = exec_ctx.symbol_table.get('list')

        if not isinstance(list_obj, List):
            return RuntimeResult().failure(RunTimeError(
                self.pos_start, self.pos_end, 'First argument is not type list', exec_ctx
            ))

        list_obj.elements.reverse()
        return RuntimeResult().success(Number.null)

    def execute_reversed(self, exec_ctx):
        list_obj = exec_ctx.symbol_table.get('list')

        if not isinstance(list_obj, List):
            return RuntimeResult().failure(RunTimeError(
                self.pos_start, self.pos_end, 'First argument is not type list', exec_ctx
            ))

        elements = list_obj.elements
        elements.reverse()

        return RuntimeResult().success(List(elements))

    def execute_clear(self, exec_ctx):
        list_obj = exec_ctx.symbol_table.get('list')

        if not isinstance(list_obj, List):
            return RuntimeResult().failure(RunTimeError(
                self.pos_start, self.pos_end, 'First argument is not type list', exec_ctx
            ))

        list_obj.elements.clear()
        return RuntimeResult().success(Number.null)

    def execute_length(self, exec_ctx):
        obj = exec_ctx.symbol_table.get('obj')

        if isinstance(obj, List):
            return RuntimeResult().success(Number(len(obj.elements)))

        elif isinstance(obj, String):
            return RuntimeResult().success(Number(len(obj.value)))

        else:
            return RuntimeResult().failure(RunTimeError(
                self.pos_start, self.pos_end, 'First argument is not type list or string', exec_ctx
            ))

    execute_print.arg_names = ['value']
    execute_input.arg_names = []
    execute_isnumber.arg_names = ['value']
    execute_isstring.arg_names = ['value']
    execute_islist.arg_names = ['value']
    execute_isbasefunction.arg_names = ['value']
    execute_append.arg_names = ['list', 'value']
    execute_pop.arg_names = ['list', 'value']
    execute_extend.arg_names = ['list_a', 'list_b']
    execute_reverse.arg_names = ['list']
    execute_reversed.arg_names = ['list']
    execute_clear.arg_names = ['list']
    execute_length.arg_names = ['obj']
