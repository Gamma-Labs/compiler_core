


struct File{
    declarations:std::Vec<Declaration>;
}

enum Declaration{
    FunctionDeclaration(Identifier,Parameters,Type,Scope)
}
struct Identifier{
    name:String
}
