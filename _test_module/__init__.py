class ParserForImport:
    def init(self, options):
        pass
    def parse(self, logmsg, message):
        print("foo")
        print("bar")
        print(message)
        logmsg["foo"] = "bar"
        print(logmsg["foo"])
        return True

# Keep this class commented out
# class NonExistingParser: pass
class ExistingParser: pass

class CallableClass: pass

NotCallableObject = int()

class ClassWithInitMethod:
    def init(self, options):
        pass

class InitMethodReturnsNotNone:
    def init(self, options):
        return True

class ParserWithoutInitMethod: pass
