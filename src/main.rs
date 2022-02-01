use std::borrow::Borrow;
use std::fmt::Write;
use std::rc::Rc;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::{InputStream, TidExt};
use antlr_rust::parser::ParserNodeType;
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::RuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{ErrorNode, NodeText, ParseTree, ParseTreeListener, ParseTreeVisitor, TerminalNode, Tree, VisitChildren};
use cql_rust::cqllexer::CqlLexer;
use cql_rust::cqlparser::{AggregateContext, AllowFilteringSpecContext, AlterKeyspaceContext, AlterMaterializedViewContext, AlterRoleContext, AlterTableAddContext, AlterTableColumnDefinitionContext, AlterTableContext, AlterTableDropColumnListContext, AlterTableDropColumnsContext, AlterTableDropCompactStorageContext, AlterTableOperationContext, AlterTableRenameContext, AlterTableWithContext, AlterTypeAddContext, AlterTypeAlterTypeContext, AlterTypeContext, AlterTypeOperationContext, AlterTypeRenameContext, AlterTypeRenameItemContext, AlterTypeRenameListContext, AlterUserContext, ApplyBatchContext, AssignmentElementContext, AssignmentListContext, AssignmentMapContext, AssignmentsContext, AssignmentSetContext, AssignmentTupleContext, BatchTypeContext, BeginBatchContext, BooleanLiteralContext, ClusteringKeyContext, ClusteringKeyListContext, ClusteringOrderContext, CodeBlockContext, ColumnContext, ColumnDefinitionContext, ColumnDefinitionListContext, ColumnListContext, ColumnNotNullContext, ColumnNotNullListContext, CompositeKeyContext, CompoundKeyContext, ConstantContext, CqlContext, CqlContextAttrs, CqlParser, CqlParserContext, CqlParserContextType, CqlParserTreeWalker, CqlsContext, CqlsContextAttrs, CreateAggregateContext, CreateFunctionContext, CreateIndexContext, CreateIndexContextExt, CreateKeyspaceContext, CreateMaterializedViewContext, CreateRoleContext, CreateTableContext, CreateTriggerContext, CreateTypeContext, CreateUserContext, DataTypeContext, DataTypeDefinitionContext, DataTypeNameContext, DecimalLiteralContext, Delete_Context, DeleteColumnItemContext, DeleteColumnListContext, DistinctSpecContext, DropAggregateContext, DropFunctionContext, DropIndexContext, DropKeyspaceContext, DropMaterializedViewContext, DropRoleContext, DropTableContext, DropTriggerContext, DropTypeContext, DropUserContext, DurableWritesContext, Empty_Context, EofContext, ExpressionContext, ExpressionListContext, FloatLiteralContext, FromSpecContext, FromSpecElementContext, Function_Context, FunctionArgsContext, FunctionCallContext, GrantContext, HashKeyContext, HexadecimalLiteralContext, IfConditionContext, IfConditionListContext, IfExistContext, IfNotExistContext, IfSpecContext, IndexColumnSpecContext, IndexEntriesSSpecContext, IndexFullSpecContext, IndexKeysSpecContext, IndexNameContext, InitCondDefinitionContext, InitCondHashContext, InitCondHashItemContext, InitCondListContext, InitCondListNestedContext, InsertColumnSpecContext, InsertContext, InsertValuesSpecContext, K_CREATE, K_INDEX, K_SELECT, KeyspaceContext, KwAddContext, KwAggregateContext, KwAllContext, KwAllowContext, KwAllPermissionsContext, KwAlterContext, KwAndContext, KwApplyContext, KwAscContext, KwAsContext, KwAuthorizeContext, KwBatchContext, KwBeginContext, KwByContext, KwCalledContext, KwClusteringContext, KwCompactContext, KwContainsContext, KwCreateContext, KwCreateContextAttrs, KwDeleteContext, KwDescContext, KwDescibeContext, KwDistinctContext, KwDropContext, KwDurableWritesContext, KwEntriesContext, KwExecuteContext, KwExistsContext, KwFilteringContext, KwFinalfuncContext, KwFromContext, KwFullContext, KwFunctionContext, KwFunctionsContext, KwGrantContext, KwIfContext, KwInContext, KwIndexContext, KwInitcondContext, KwInputContext, KwInsertContext, KwIntoContext, KwIsContext, KwJsonContext, KwKeyContext, KwKeysContext, KwKeyspaceContext, KwKeyspacesContext, KwLanguageContext, KwLimitContext, KwListContext, KwLoggedContext, KwLoginContext, KwMaterializedContext, KwModifyContext, KwNorecursiveContext, KwNosuperuserContext, KwNotContext, KwNullContext, KwOfContext, KwOnContext, KwOptionsContext, KwOrContext, KwOrderContext, KwPasswordContext, KwPrimaryContext, KwRenameContext, KwReplaceContext, KwReplicationContext, KwReturnsContext, KwRevokeContext, KwRoleContext, KwRolesContext, KwSelectContext, KwSelectContextAttrs, KwSetContext, KwSfuncContext, KwStorageContext, KwStypeContext, KwSuperuserContext, KwTableContext, KwTimestampContext, KwToContext, KwTriggerContext, KwTruncateContext, KwTtlContext, KwTypeContext, KwUnloggedContext, KwUpdateContext, KwUseContext, KwUserContext, KwUsersContext, KwUsingContext, KwValuesContext, KwViewContext, KwWhereContext, KwWithContext, LanguageContext, LimitSpecContext, ListPermissionsContext, ListRolesContext, ListUsersContext, MaterializedViewContext, MaterializedViewOptionsContext, MaterializedViewWhereContext, OptionHashContext, OptionHashItemContext, OptionHashKeyContext, OptionHashValueContext, OrderDirectionContext, OrderSpecContext, OrderSpecElementContext, OrReplaceContext, ParamContext, ParamListContext, ParamNameContext, PartitionKeyContext, PartitionKeyListContext, PasswordContext, PrimaryKeyColumnContext, PrimaryKeyDefinitionContext, PrimaryKeyElementContext, PriviledgeContext, RelalationContainsContext, RelalationContainsKeyContext, RelationElementContext, RelationElementsContext, ReplicationListContext, ReplicationListItemContext, ResourceContext, ReturnModeContext, RevokeContext, RoleContext, RoleWithContext, RoleWithOptionsContext, RootContext, RootContextAll, RootContextAttrs, RULE_kwCreate, Select_Context, Select_ContextAttrs, SelectElementContext, SelectElementContextAttrs, SelectElementContextExt, SelectElementsContext, SelectElementsContextAttrs, SinglePrimaryKeyContext, StatementSeparatorContext, StringLiteralContext, SyntaxBracketLaContext, SyntaxBracketLcContext, SyntaxBracketLrContext, SyntaxBracketLsContext, SyntaxBracketRaContext, SyntaxBracketRcContext, SyntaxBracketRrContext, SyntaxBracketRsContext, SyntaxColonContext, SyntaxCommaContext, TableContext, TableOptionItemContext, TableOptionNameContext, TableOptionsContext, TableOptionValueContext, TimestampContext, TriggerClassContext, TriggerContext, TruncateContext, TtlContext, Type_Context, TypeMemberColumnListContext, UpdateContext, Use_Context, UserContext, UserPasswordContext, UserSuperUserContext, UsingTimestampSpecContext, UsingTtlTimestampContext, WhereSpecContext, WithElementContext};
use cql_rust::cqlparservisitor::CqlParserVisitor;
use antlr_rust::tree::Visitable;
use cql_rust::cqlparser;
use cql_rust::cqlparserlistener::CqlParserListener;

struct MyCQLVisitor<'i> {
    create : Option<Rc<TerminalNode<'i, CqlParserContextType>>>,
    select : Vec<Box<SelectElementContext<'i>>>,
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
        //self.select = ctx.K_SELECT();
        self.visit_children( ctx )
    }

    fn visit_selectElement(&mut self, ctx: &SelectElementContext<'i>) {
        //let y : &'i SelectElementContext<'i> = ctx.clone();
        println!( "Select element {:?}", ctx.get_text());
        let f = ctx.clone();
        let g = f.get_children();
        g.for_each( |h| println!( "select element child {:?}", h.get_text()));
        self.visit_children( ctx );
    }

    fn visit_column(&mut self, ctx: &ColumnContext<'i>) {
        println!( "column {:?}", ctx);
    }

}

impl<'input> ParseTreeVisitor<'input, CqlParserContextType> for MyCQLVisitor<'input> {

}

fn main() {
    println!("Hello, world!");

    struct Listener4 {
        token_indices : Vec<isize>,
        data: String,
    }

    impl<'input> ParseTreeListener<'input, CqlParserContextType> for Listener4 {
        fn visit_terminal(&mut self, node: &TerminalNode<'input, CqlParserContextType>) {
/*            println!("enter terminal {}",node.symbol.get_text()); */
            self.token_indices.push(node.symbol.get_token_index());
            let _ = write!(&mut self.data, " {}", node.symbol.get_text());
        }
        fn enter_every_rule(&mut self, ctx: &dyn CqlParserContext<'input>) {
          /*  println!(
                "rule entered {}",
                cqlparser::ruleNames
                    .get(ctx.get_rule_index())
                    .unwrap_or(&"error")
            )

           */
        }
    }

    impl<'input> CqlParserListener<'input> for Listener4 {}

    let input = InputStream::new("SELECT col1 as foo, col2 FROM cycling.cyclist_cat_pts WHERE category = 'GC' ORDER BY points ASC;");
    let lexer = CqlLexer::new( input);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = CqlParser::new( token_stream );
    let id = parser.add_parse_listener(Box::new(Listener4 {
        token_indices: vec![],
        data: String::new(),
    }));
    let result = parser.root().expect("parsed unsuccessfully");
    let mut visitor = MyCQLVisitor{
        create: None,
        select : vec![],
    };
    result.accept( &mut visitor );

    //let x = parser.get_rule_names();
    //x.into_iter().for_each( |x| println!( "{:?}", x));

    println!( "{:?}", result.to_string_tree(&*parser) );


    println!( "Child count: {:?}", result.get_child_count());
    let y = result.get_child(0).unwrap();
    println!( "{:?}", y.to_string_tree(&*parser ) );

    //let select = visitor.select.as_ref().unwrap().clone();
    //println!( "select {:?}", select.to_string_tree(&*parser ) );

    //println!( "select symbol {:?}",select.symbol.get_text() );
    //println!( "select is {:?}",select.symbol );


    let mut listener = parser.remove_parse_listener(id);
    println!( "listener {:?}", &listener.data );

    let listener2 = CqlParserTreeWalker::walk(Box::new(Listener4 {
        token_indices: vec![],
        data: String::new(),
    }), &*result);
    print!( "listener2 " );
    println!( "{}", &listener2.data );
    println!( "{:?}", &listener2.token_indices);
    //for i in listener2.token_indices {
    //    println!("{:?}", result.child_of_type::<SelectElementContext>(0));
    //}
    let cql = result.cqls().unwrap().cql(0).unwrap();
    let elements = cql.select_().unwrap().selectElements().unwrap().selectElement_all();
    //elements.iter().for_each( |e| e.accept( &mut visitor ));
    for x in visitor.select {
        println!( "{:?}", x.get_node_text(&cqlparser::ruleNames) );
    }
}

