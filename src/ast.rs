struct File {
    declarations: std::Vec<Declaration>,
}

enum Declaration {
    FunctionDeclaration(Identifier, TemplateParameters, Parameters, Type, CompoundStatement),
}

struct TemplateParameters {
    lists: std::Vec<std::Vec<Parameter>>,
}

struct Parameters {
    lists: std::Vec<std::Vec<Parameter>>,
}

struct Type {

}

struct Identifier {
    name: String,
}
