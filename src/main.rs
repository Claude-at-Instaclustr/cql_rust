use std::rc::Rc;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;
use antlr_rust::parser::ParserNodeType;
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::tree::{ErrorNode, NodeText, ParseTree, ParseTreeVisitor, TerminalNode, Tree, VisitChildren};
use cql_rust::cqllexer::CqlLexer;
use cql_rust::cqlparser::{AggregateContext, AllowFilteringSpecContext, AlterKeyspaceContext, AlterMaterializedViewContext, AlterRoleContext, AlterTableAddContext, AlterTableColumnDefinitionContext, AlterTableContext, AlterTableDropColumnListContext, AlterTableDropColumnsContext, AlterTableDropCompactStorageContext, AlterTableOperationContext, AlterTableRenameContext, AlterTableWithContext, AlterTypeAddContext, AlterTypeAlterTypeContext, AlterTypeContext, AlterTypeOperationContext, AlterTypeRenameContext, AlterTypeRenameItemContext, AlterTypeRenameListContext, AlterUserContext, ApplyBatchContext, AssignmentElementContext, AssignmentListContext, AssignmentMapContext, AssignmentsContext, AssignmentSetContext, AssignmentTupleContext, BatchTypeContext, BeginBatchContext, BooleanLiteralContext, ClusteringKeyContext, ClusteringKeyListContext, ClusteringOrderContext, CodeBlockContext, ColumnContext, ColumnDefinitionContext, ColumnDefinitionListContext, ColumnListContext, ColumnNotNullContext, ColumnNotNullListContext, CompositeKeyContext, CompoundKeyContext, ConstantContext, CqlContext, CqlParser, CqlParserContext, CqlParserContextType, CqlsContext, CreateAggregateContext, CreateFunctionContext, CreateIndexContext, CreateIndexContextExt, CreateKeyspaceContext, CreateMaterializedViewContext, CreateRoleContext, CreateTableContext, CreateTriggerContext, CreateTypeContext, CreateUserContext, DataTypeContext, DataTypeDefinitionContext, DataTypeNameContext, DecimalLiteralContext, Delete_Context, DeleteColumnItemContext, DeleteColumnListContext, DistinctSpecContext, DropAggregateContext, DropFunctionContext, DropIndexContext, DropKeyspaceContext, DropMaterializedViewContext, DropRoleContext, DropTableContext, DropTriggerContext, DropTypeContext, DropUserContext, DurableWritesContext, Empty_Context, EofContext, ExpressionContext, ExpressionListContext, FloatLiteralContext, FromSpecContext, FromSpecElementContext, Function_Context, FunctionArgsContext, FunctionCallContext, GrantContext, HashKeyContext, HexadecimalLiteralContext, IfConditionContext, IfConditionListContext, IfExistContext, IfNotExistContext, IfSpecContext, IndexColumnSpecContext, IndexEntriesSSpecContext, IndexFullSpecContext, IndexKeysSpecContext, IndexNameContext, InitCondDefinitionContext, InitCondHashContext, InitCondHashItemContext, InitCondListContext, InitCondListNestedContext, InsertColumnSpecContext, InsertContext, InsertValuesSpecContext, K_CREATE, K_INDEX, K_SELECT, KeyspaceContext, KwAddContext, KwAggregateContext, KwAllContext, KwAllowContext, KwAllPermissionsContext, KwAlterContext, KwAndContext, KwApplyContext, KwAscContext, KwAsContext, KwAuthorizeContext, KwBatchContext, KwBeginContext, KwByContext, KwCalledContext, KwClusteringContext, KwCompactContext, KwContainsContext, KwCreateContext, KwCreateContextAttrs, KwDeleteContext, KwDescContext, KwDescibeContext, KwDistinctContext, KwDropContext, KwDurableWritesContext, KwEntriesContext, KwExecuteContext, KwExistsContext, KwFilteringContext, KwFinalfuncContext, KwFromContext, KwFullContext, KwFunctionContext, KwFunctionsContext, KwGrantContext, KwIfContext, KwInContext, KwIndexContext, KwInitcondContext, KwInputContext, KwInsertContext, KwIntoContext, KwIsContext, KwJsonContext, KwKeyContext, KwKeysContext, KwKeyspaceContext, KwKeyspacesContext, KwLanguageContext, KwLimitContext, KwListContext, KwLoggedContext, KwLoginContext, KwMaterializedContext, KwModifyContext, KwNorecursiveContext, KwNosuperuserContext, KwNotContext, KwNullContext, KwOfContext, KwOnContext, KwOptionsContext, KwOrContext, KwOrderContext, KwPasswordContext, KwPrimaryContext, KwRenameContext, KwReplaceContext, KwReplicationContext, KwReturnsContext, KwRevokeContext, KwRoleContext, KwRolesContext, KwSelectContext, KwSelectContextAttrs, KwSetContext, KwSfuncContext, KwStorageContext, KwStypeContext, KwSuperuserContext, KwTableContext, KwTimestampContext, KwToContext, KwTriggerContext, KwTruncateContext, KwTtlContext, KwTypeContext, KwUnloggedContext, KwUpdateContext, KwUseContext, KwUserContext, KwUsersContext, KwUsingContext, KwValuesContext, KwViewContext, KwWhereContext, KwWithContext, LanguageContext, LimitSpecContext, ListPermissionsContext, ListRolesContext, ListUsersContext, MaterializedViewContext, MaterializedViewOptionsContext, MaterializedViewWhereContext, OptionHashContext, OptionHashItemContext, OptionHashKeyContext, OptionHashValueContext, OrderDirectionContext, OrderSpecContext, OrderSpecElementContext, OrReplaceContext, ParamContext, ParamListContext, ParamNameContext, PartitionKeyContext, PartitionKeyListContext, PasswordContext, PrimaryKeyColumnContext, PrimaryKeyDefinitionContext, PrimaryKeyElementContext, PriviledgeContext, RelalationContainsContext, RelalationContainsKeyContext, RelationElementContext, RelationElementsContext, ReplicationListContext, ReplicationListItemContext, ResourceContext, ReturnModeContext, RevokeContext, RoleContext, RoleWithContext, RoleWithOptionsContext, RootContext, RootContextAll, RULE_kwCreate, Select_Context, SelectElementContext, SelectElementsContext, SinglePrimaryKeyContext, StatementSeparatorContext, StringLiteralContext, SyntaxBracketLaContext, SyntaxBracketLcContext, SyntaxBracketLrContext, SyntaxBracketLsContext, SyntaxBracketRaContext, SyntaxBracketRcContext, SyntaxBracketRrContext, SyntaxBracketRsContext, SyntaxColonContext, SyntaxCommaContext, TableContext, TableOptionItemContext, TableOptionNameContext, TableOptionsContext, TableOptionValueContext, TimestampContext, TriggerClassContext, TriggerContext, TruncateContext, TtlContext, Type_Context, TypeMemberColumnListContext, UpdateContext, Use_Context, UserContext, UserPasswordContext, UserSuperUserContext, UsingTimestampSpecContext, UsingTtlTimestampContext, WhereSpecContext, WithElementContext};
use cql_rust::cqlparservisitor::CqlParserVisitor;
use antlr_rust::tree::Visitable;

struct MyCQLVisitor<'i> {
    create : Option<Rc<TerminalNode<'i, CqlParserContextType>>>,
    select : Option<Rc<TerminalNode<'i, CqlParserContextType>>>,
}


impl<'i> CqlParserVisitor<'i> for MyCQLVisitor<'i> {
    fn visit_cql(&mut self, ctx: &CqlContext<'i>) {
        println!( "{:?}", ctx.get_text() );
        self.visit_children( ctx )
    }

    fn visit_createIndex(&mut self, ctx: &CreateIndexContext<'i>) {
        self.visit_children( ctx )
    }

    fn visit_kwCreate(&mut self, ctx: &KwCreateContext<'i>) {
        self.create = ctx.K_CREATE();
    }

    fn visit_kwSelect(&mut self, ctx: &KwSelectContext<'i>) {
        self.select = ctx.K_SELECT();
        self.visit_children( ctx )
    }

}

impl<'input> ParseTreeVisitor<'input, CqlParserContextType> for MyCQLVisitor<'input> {

}

fn main() {
    println!("Hello, world!");

   let input = InputStream::new("SELECT * FROM cycling.cyclist_cat_pts WHERE category = 'GC' ORDER BY points ASC;");
    let lexer = CqlLexer::new( input);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = CqlParser::new( token_stream );
    let result = parser.root().expect("parsed unsuccessfully");
    let mut visitor = MyCQLVisitor{
        create: None,
        select : None,
    };
    result.accept( &mut visitor );

    //let x = parser.get_rule_names();
    //x.into_iter().for_each( |x| println!( "{:?}", x));

    println!( "{:?}", result.to_string_tree(&*parser) );

    let n = result;
    println!( "Child count: {:?}", n.get_child_count());
    let y = n.get_child(0).unwrap();
    println!( "{:?}", y.to_string_tree(&*parser ) );

    let z = visitor.select.unwrap();
    println!( "{:?}", z.to_string_tree(&*parser ) );

}

