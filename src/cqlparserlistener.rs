#![allow(nonstandard_style)]
// Generated from grammars-v4/cql3/CqlParser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::cqlparser::*;

pub trait CqlParserListener<'input> : ParseTreeListener<'input,CqlParserContextType>{

/**
 * Enter a parse tree produced by {@link CqlParser#root}.
 * @param ctx the parse tree
 */
fn enter_root(&mut self, _ctx: &RootContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#root}.
 * @param ctx the parse tree
 */
fn exit_root(&mut self, _ctx: &RootContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#cqls}.
 * @param ctx the parse tree
 */
fn enter_cqls(&mut self, _ctx: &CqlsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#cqls}.
 * @param ctx the parse tree
 */
fn exit_cqls(&mut self, _ctx: &CqlsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#statementSeparator}.
 * @param ctx the parse tree
 */
fn enter_statementSeparator(&mut self, _ctx: &StatementSeparatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#statementSeparator}.
 * @param ctx the parse tree
 */
fn exit_statementSeparator(&mut self, _ctx: &StatementSeparatorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#empty_}.
 * @param ctx the parse tree
 */
fn enter_empty_(&mut self, _ctx: &Empty_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#empty_}.
 * @param ctx the parse tree
 */
fn exit_empty_(&mut self, _ctx: &Empty_Context<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#cql}.
 * @param ctx the parse tree
 */
fn enter_cql(&mut self, _ctx: &CqlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#cql}.
 * @param ctx the parse tree
 */
fn exit_cql(&mut self, _ctx: &CqlContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#revoke}.
 * @param ctx the parse tree
 */
fn enter_revoke(&mut self, _ctx: &RevokeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#revoke}.
 * @param ctx the parse tree
 */
fn exit_revoke(&mut self, _ctx: &RevokeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#listUsers}.
 * @param ctx the parse tree
 */
fn enter_listUsers(&mut self, _ctx: &ListUsersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#listUsers}.
 * @param ctx the parse tree
 */
fn exit_listUsers(&mut self, _ctx: &ListUsersContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#listRoles}.
 * @param ctx the parse tree
 */
fn enter_listRoles(&mut self, _ctx: &ListRolesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#listRoles}.
 * @param ctx the parse tree
 */
fn exit_listRoles(&mut self, _ctx: &ListRolesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#listPermissions}.
 * @param ctx the parse tree
 */
fn enter_listPermissions(&mut self, _ctx: &ListPermissionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#listPermissions}.
 * @param ctx the parse tree
 */
fn exit_listPermissions(&mut self, _ctx: &ListPermissionsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#grant}.
 * @param ctx the parse tree
 */
fn enter_grant(&mut self, _ctx: &GrantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#grant}.
 * @param ctx the parse tree
 */
fn exit_grant(&mut self, _ctx: &GrantContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#priviledge}.
 * @param ctx the parse tree
 */
fn enter_priviledge(&mut self, _ctx: &PriviledgeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#priviledge}.
 * @param ctx the parse tree
 */
fn exit_priviledge(&mut self, _ctx: &PriviledgeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#resource}.
 * @param ctx the parse tree
 */
fn enter_resource(&mut self, _ctx: &ResourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#resource}.
 * @param ctx the parse tree
 */
fn exit_resource(&mut self, _ctx: &ResourceContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#createUser}.
 * @param ctx the parse tree
 */
fn enter_createUser(&mut self, _ctx: &CreateUserContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#createUser}.
 * @param ctx the parse tree
 */
fn exit_createUser(&mut self, _ctx: &CreateUserContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#createRole}.
 * @param ctx the parse tree
 */
fn enter_createRole(&mut self, _ctx: &CreateRoleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#createRole}.
 * @param ctx the parse tree
 */
fn exit_createRole(&mut self, _ctx: &CreateRoleContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#createType}.
 * @param ctx the parse tree
 */
fn enter_createType(&mut self, _ctx: &CreateTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#createType}.
 * @param ctx the parse tree
 */
fn exit_createType(&mut self, _ctx: &CreateTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#typeMemberColumnList}.
 * @param ctx the parse tree
 */
fn enter_typeMemberColumnList(&mut self, _ctx: &TypeMemberColumnListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#typeMemberColumnList}.
 * @param ctx the parse tree
 */
fn exit_typeMemberColumnList(&mut self, _ctx: &TypeMemberColumnListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#createTrigger}.
 * @param ctx the parse tree
 */
fn enter_createTrigger(&mut self, _ctx: &CreateTriggerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#createTrigger}.
 * @param ctx the parse tree
 */
fn exit_createTrigger(&mut self, _ctx: &CreateTriggerContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#createMaterializedView}.
 * @param ctx the parse tree
 */
fn enter_createMaterializedView(&mut self, _ctx: &CreateMaterializedViewContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#createMaterializedView}.
 * @param ctx the parse tree
 */
fn exit_createMaterializedView(&mut self, _ctx: &CreateMaterializedViewContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#materializedViewWhere}.
 * @param ctx the parse tree
 */
fn enter_materializedViewWhere(&mut self, _ctx: &MaterializedViewWhereContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#materializedViewWhere}.
 * @param ctx the parse tree
 */
fn exit_materializedViewWhere(&mut self, _ctx: &MaterializedViewWhereContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#columnNotNullList}.
 * @param ctx the parse tree
 */
fn enter_columnNotNullList(&mut self, _ctx: &ColumnNotNullListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#columnNotNullList}.
 * @param ctx the parse tree
 */
fn exit_columnNotNullList(&mut self, _ctx: &ColumnNotNullListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#columnNotNull}.
 * @param ctx the parse tree
 */
fn enter_columnNotNull(&mut self, _ctx: &ColumnNotNullContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#columnNotNull}.
 * @param ctx the parse tree
 */
fn exit_columnNotNull(&mut self, _ctx: &ColumnNotNullContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#materializedViewOptions}.
 * @param ctx the parse tree
 */
fn enter_materializedViewOptions(&mut self, _ctx: &MaterializedViewOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#materializedViewOptions}.
 * @param ctx the parse tree
 */
fn exit_materializedViewOptions(&mut self, _ctx: &MaterializedViewOptionsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#createKeyspace}.
 * @param ctx the parse tree
 */
fn enter_createKeyspace(&mut self, _ctx: &CreateKeyspaceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#createKeyspace}.
 * @param ctx the parse tree
 */
fn exit_createKeyspace(&mut self, _ctx: &CreateKeyspaceContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#createFunction}.
 * @param ctx the parse tree
 */
fn enter_createFunction(&mut self, _ctx: &CreateFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#createFunction}.
 * @param ctx the parse tree
 */
fn exit_createFunction(&mut self, _ctx: &CreateFunctionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#codeBlock}.
 * @param ctx the parse tree
 */
fn enter_codeBlock(&mut self, _ctx: &CodeBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#codeBlock}.
 * @param ctx the parse tree
 */
fn exit_codeBlock(&mut self, _ctx: &CodeBlockContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#paramList}.
 * @param ctx the parse tree
 */
fn enter_paramList(&mut self, _ctx: &ParamListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#paramList}.
 * @param ctx the parse tree
 */
fn exit_paramList(&mut self, _ctx: &ParamListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#returnMode}.
 * @param ctx the parse tree
 */
fn enter_returnMode(&mut self, _ctx: &ReturnModeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#returnMode}.
 * @param ctx the parse tree
 */
fn exit_returnMode(&mut self, _ctx: &ReturnModeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#createAggregate}.
 * @param ctx the parse tree
 */
fn enter_createAggregate(&mut self, _ctx: &CreateAggregateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#createAggregate}.
 * @param ctx the parse tree
 */
fn exit_createAggregate(&mut self, _ctx: &CreateAggregateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#initCondDefinition}.
 * @param ctx the parse tree
 */
fn enter_initCondDefinition(&mut self, _ctx: &InitCondDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#initCondDefinition}.
 * @param ctx the parse tree
 */
fn exit_initCondDefinition(&mut self, _ctx: &InitCondDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#initCondHash}.
 * @param ctx the parse tree
 */
fn enter_initCondHash(&mut self, _ctx: &InitCondHashContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#initCondHash}.
 * @param ctx the parse tree
 */
fn exit_initCondHash(&mut self, _ctx: &InitCondHashContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#initCondHashItem}.
 * @param ctx the parse tree
 */
fn enter_initCondHashItem(&mut self, _ctx: &InitCondHashItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#initCondHashItem}.
 * @param ctx the parse tree
 */
fn exit_initCondHashItem(&mut self, _ctx: &InitCondHashItemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#initCondListNested}.
 * @param ctx the parse tree
 */
fn enter_initCondListNested(&mut self, _ctx: &InitCondListNestedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#initCondListNested}.
 * @param ctx the parse tree
 */
fn exit_initCondListNested(&mut self, _ctx: &InitCondListNestedContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#initCondList}.
 * @param ctx the parse tree
 */
fn enter_initCondList(&mut self, _ctx: &InitCondListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#initCondList}.
 * @param ctx the parse tree
 */
fn exit_initCondList(&mut self, _ctx: &InitCondListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#orReplace}.
 * @param ctx the parse tree
 */
fn enter_orReplace(&mut self, _ctx: &OrReplaceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#orReplace}.
 * @param ctx the parse tree
 */
fn exit_orReplace(&mut self, _ctx: &OrReplaceContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterUser}.
 * @param ctx the parse tree
 */
fn enter_alterUser(&mut self, _ctx: &AlterUserContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterUser}.
 * @param ctx the parse tree
 */
fn exit_alterUser(&mut self, _ctx: &AlterUserContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#userPassword}.
 * @param ctx the parse tree
 */
fn enter_userPassword(&mut self, _ctx: &UserPasswordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#userPassword}.
 * @param ctx the parse tree
 */
fn exit_userPassword(&mut self, _ctx: &UserPasswordContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#userSuperUser}.
 * @param ctx the parse tree
 */
fn enter_userSuperUser(&mut self, _ctx: &UserSuperUserContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#userSuperUser}.
 * @param ctx the parse tree
 */
fn exit_userSuperUser(&mut self, _ctx: &UserSuperUserContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterType}.
 * @param ctx the parse tree
 */
fn enter_alterType(&mut self, _ctx: &AlterTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterType}.
 * @param ctx the parse tree
 */
fn exit_alterType(&mut self, _ctx: &AlterTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTypeOperation}.
 * @param ctx the parse tree
 */
fn enter_alterTypeOperation(&mut self, _ctx: &AlterTypeOperationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTypeOperation}.
 * @param ctx the parse tree
 */
fn exit_alterTypeOperation(&mut self, _ctx: &AlterTypeOperationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTypeRename}.
 * @param ctx the parse tree
 */
fn enter_alterTypeRename(&mut self, _ctx: &AlterTypeRenameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTypeRename}.
 * @param ctx the parse tree
 */
fn exit_alterTypeRename(&mut self, _ctx: &AlterTypeRenameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTypeRenameList}.
 * @param ctx the parse tree
 */
fn enter_alterTypeRenameList(&mut self, _ctx: &AlterTypeRenameListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTypeRenameList}.
 * @param ctx the parse tree
 */
fn exit_alterTypeRenameList(&mut self, _ctx: &AlterTypeRenameListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTypeRenameItem}.
 * @param ctx the parse tree
 */
fn enter_alterTypeRenameItem(&mut self, _ctx: &AlterTypeRenameItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTypeRenameItem}.
 * @param ctx the parse tree
 */
fn exit_alterTypeRenameItem(&mut self, _ctx: &AlterTypeRenameItemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTypeAdd}.
 * @param ctx the parse tree
 */
fn enter_alterTypeAdd(&mut self, _ctx: &AlterTypeAddContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTypeAdd}.
 * @param ctx the parse tree
 */
fn exit_alterTypeAdd(&mut self, _ctx: &AlterTypeAddContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTypeAlterType}.
 * @param ctx the parse tree
 */
fn enter_alterTypeAlterType(&mut self, _ctx: &AlterTypeAlterTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTypeAlterType}.
 * @param ctx the parse tree
 */
fn exit_alterTypeAlterType(&mut self, _ctx: &AlterTypeAlterTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTable}.
 * @param ctx the parse tree
 */
fn enter_alterTable(&mut self, _ctx: &AlterTableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTable}.
 * @param ctx the parse tree
 */
fn exit_alterTable(&mut self, _ctx: &AlterTableContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTableOperation}.
 * @param ctx the parse tree
 */
fn enter_alterTableOperation(&mut self, _ctx: &AlterTableOperationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTableOperation}.
 * @param ctx the parse tree
 */
fn exit_alterTableOperation(&mut self, _ctx: &AlterTableOperationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTableWith}.
 * @param ctx the parse tree
 */
fn enter_alterTableWith(&mut self, _ctx: &AlterTableWithContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTableWith}.
 * @param ctx the parse tree
 */
fn exit_alterTableWith(&mut self, _ctx: &AlterTableWithContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTableRename}.
 * @param ctx the parse tree
 */
fn enter_alterTableRename(&mut self, _ctx: &AlterTableRenameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTableRename}.
 * @param ctx the parse tree
 */
fn exit_alterTableRename(&mut self, _ctx: &AlterTableRenameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTableDropCompactStorage}.
 * @param ctx the parse tree
 */
fn enter_alterTableDropCompactStorage(&mut self, _ctx: &AlterTableDropCompactStorageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTableDropCompactStorage}.
 * @param ctx the parse tree
 */
fn exit_alterTableDropCompactStorage(&mut self, _ctx: &AlterTableDropCompactStorageContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTableDropColumns}.
 * @param ctx the parse tree
 */
fn enter_alterTableDropColumns(&mut self, _ctx: &AlterTableDropColumnsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTableDropColumns}.
 * @param ctx the parse tree
 */
fn exit_alterTableDropColumns(&mut self, _ctx: &AlterTableDropColumnsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTableDropColumnList}.
 * @param ctx the parse tree
 */
fn enter_alterTableDropColumnList(&mut self, _ctx: &AlterTableDropColumnListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTableDropColumnList}.
 * @param ctx the parse tree
 */
fn exit_alterTableDropColumnList(&mut self, _ctx: &AlterTableDropColumnListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTableAdd}.
 * @param ctx the parse tree
 */
fn enter_alterTableAdd(&mut self, _ctx: &AlterTableAddContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTableAdd}.
 * @param ctx the parse tree
 */
fn exit_alterTableAdd(&mut self, _ctx: &AlterTableAddContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterTableColumnDefinition}.
 * @param ctx the parse tree
 */
fn enter_alterTableColumnDefinition(&mut self, _ctx: &AlterTableColumnDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterTableColumnDefinition}.
 * @param ctx the parse tree
 */
fn exit_alterTableColumnDefinition(&mut self, _ctx: &AlterTableColumnDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterRole}.
 * @param ctx the parse tree
 */
fn enter_alterRole(&mut self, _ctx: &AlterRoleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterRole}.
 * @param ctx the parse tree
 */
fn exit_alterRole(&mut self, _ctx: &AlterRoleContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#roleWith}.
 * @param ctx the parse tree
 */
fn enter_roleWith(&mut self, _ctx: &RoleWithContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#roleWith}.
 * @param ctx the parse tree
 */
fn exit_roleWith(&mut self, _ctx: &RoleWithContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#roleWithOptions}.
 * @param ctx the parse tree
 */
fn enter_roleWithOptions(&mut self, _ctx: &RoleWithOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#roleWithOptions}.
 * @param ctx the parse tree
 */
fn exit_roleWithOptions(&mut self, _ctx: &RoleWithOptionsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterMaterializedView}.
 * @param ctx the parse tree
 */
fn enter_alterMaterializedView(&mut self, _ctx: &AlterMaterializedViewContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterMaterializedView}.
 * @param ctx the parse tree
 */
fn exit_alterMaterializedView(&mut self, _ctx: &AlterMaterializedViewContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#dropUser}.
 * @param ctx the parse tree
 */
fn enter_dropUser(&mut self, _ctx: &DropUserContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#dropUser}.
 * @param ctx the parse tree
 */
fn exit_dropUser(&mut self, _ctx: &DropUserContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#dropType}.
 * @param ctx the parse tree
 */
fn enter_dropType(&mut self, _ctx: &DropTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#dropType}.
 * @param ctx the parse tree
 */
fn exit_dropType(&mut self, _ctx: &DropTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#dropMaterializedView}.
 * @param ctx the parse tree
 */
fn enter_dropMaterializedView(&mut self, _ctx: &DropMaterializedViewContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#dropMaterializedView}.
 * @param ctx the parse tree
 */
fn exit_dropMaterializedView(&mut self, _ctx: &DropMaterializedViewContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#dropAggregate}.
 * @param ctx the parse tree
 */
fn enter_dropAggregate(&mut self, _ctx: &DropAggregateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#dropAggregate}.
 * @param ctx the parse tree
 */
fn exit_dropAggregate(&mut self, _ctx: &DropAggregateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#dropFunction}.
 * @param ctx the parse tree
 */
fn enter_dropFunction(&mut self, _ctx: &DropFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#dropFunction}.
 * @param ctx the parse tree
 */
fn exit_dropFunction(&mut self, _ctx: &DropFunctionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#dropTrigger}.
 * @param ctx the parse tree
 */
fn enter_dropTrigger(&mut self, _ctx: &DropTriggerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#dropTrigger}.
 * @param ctx the parse tree
 */
fn exit_dropTrigger(&mut self, _ctx: &DropTriggerContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#dropRole}.
 * @param ctx the parse tree
 */
fn enter_dropRole(&mut self, _ctx: &DropRoleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#dropRole}.
 * @param ctx the parse tree
 */
fn exit_dropRole(&mut self, _ctx: &DropRoleContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#dropTable}.
 * @param ctx the parse tree
 */
fn enter_dropTable(&mut self, _ctx: &DropTableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#dropTable}.
 * @param ctx the parse tree
 */
fn exit_dropTable(&mut self, _ctx: &DropTableContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#dropKeyspace}.
 * @param ctx the parse tree
 */
fn enter_dropKeyspace(&mut self, _ctx: &DropKeyspaceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#dropKeyspace}.
 * @param ctx the parse tree
 */
fn exit_dropKeyspace(&mut self, _ctx: &DropKeyspaceContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#dropIndex}.
 * @param ctx the parse tree
 */
fn enter_dropIndex(&mut self, _ctx: &DropIndexContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#dropIndex}.
 * @param ctx the parse tree
 */
fn exit_dropIndex(&mut self, _ctx: &DropIndexContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#createTable}.
 * @param ctx the parse tree
 */
fn enter_createTable(&mut self, _ctx: &CreateTableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#createTable}.
 * @param ctx the parse tree
 */
fn exit_createTable(&mut self, _ctx: &CreateTableContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#withElement}.
 * @param ctx the parse tree
 */
fn enter_withElement(&mut self, _ctx: &WithElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#withElement}.
 * @param ctx the parse tree
 */
fn exit_withElement(&mut self, _ctx: &WithElementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#clusteringOrder}.
 * @param ctx the parse tree
 */
fn enter_clusteringOrder(&mut self, _ctx: &ClusteringOrderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#clusteringOrder}.
 * @param ctx the parse tree
 */
fn exit_clusteringOrder(&mut self, _ctx: &ClusteringOrderContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#tableOptions}.
 * @param ctx the parse tree
 */
fn enter_tableOptions(&mut self, _ctx: &TableOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#tableOptions}.
 * @param ctx the parse tree
 */
fn exit_tableOptions(&mut self, _ctx: &TableOptionsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#tableOptionItem}.
 * @param ctx the parse tree
 */
fn enter_tableOptionItem(&mut self, _ctx: &TableOptionItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#tableOptionItem}.
 * @param ctx the parse tree
 */
fn exit_tableOptionItem(&mut self, _ctx: &TableOptionItemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#tableOptionName}.
 * @param ctx the parse tree
 */
fn enter_tableOptionName(&mut self, _ctx: &TableOptionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#tableOptionName}.
 * @param ctx the parse tree
 */
fn exit_tableOptionName(&mut self, _ctx: &TableOptionNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#tableOptionValue}.
 * @param ctx the parse tree
 */
fn enter_tableOptionValue(&mut self, _ctx: &TableOptionValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#tableOptionValue}.
 * @param ctx the parse tree
 */
fn exit_tableOptionValue(&mut self, _ctx: &TableOptionValueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#optionHash}.
 * @param ctx the parse tree
 */
fn enter_optionHash(&mut self, _ctx: &OptionHashContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#optionHash}.
 * @param ctx the parse tree
 */
fn exit_optionHash(&mut self, _ctx: &OptionHashContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#optionHashItem}.
 * @param ctx the parse tree
 */
fn enter_optionHashItem(&mut self, _ctx: &OptionHashItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#optionHashItem}.
 * @param ctx the parse tree
 */
fn exit_optionHashItem(&mut self, _ctx: &OptionHashItemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#optionHashKey}.
 * @param ctx the parse tree
 */
fn enter_optionHashKey(&mut self, _ctx: &OptionHashKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#optionHashKey}.
 * @param ctx the parse tree
 */
fn exit_optionHashKey(&mut self, _ctx: &OptionHashKeyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#optionHashValue}.
 * @param ctx the parse tree
 */
fn enter_optionHashValue(&mut self, _ctx: &OptionHashValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#optionHashValue}.
 * @param ctx the parse tree
 */
fn exit_optionHashValue(&mut self, _ctx: &OptionHashValueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#columnDefinitionList}.
 * @param ctx the parse tree
 */
fn enter_columnDefinitionList(&mut self, _ctx: &ColumnDefinitionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#columnDefinitionList}.
 * @param ctx the parse tree
 */
fn exit_columnDefinitionList(&mut self, _ctx: &ColumnDefinitionListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#columnDefinition}.
 * @param ctx the parse tree
 */
fn enter_columnDefinition(&mut self, _ctx: &ColumnDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#columnDefinition}.
 * @param ctx the parse tree
 */
fn exit_columnDefinition(&mut self, _ctx: &ColumnDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#primaryKeyColumn}.
 * @param ctx the parse tree
 */
fn enter_primaryKeyColumn(&mut self, _ctx: &PrimaryKeyColumnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#primaryKeyColumn}.
 * @param ctx the parse tree
 */
fn exit_primaryKeyColumn(&mut self, _ctx: &PrimaryKeyColumnContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#primaryKeyElement}.
 * @param ctx the parse tree
 */
fn enter_primaryKeyElement(&mut self, _ctx: &PrimaryKeyElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#primaryKeyElement}.
 * @param ctx the parse tree
 */
fn exit_primaryKeyElement(&mut self, _ctx: &PrimaryKeyElementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#primaryKeyDefinition}.
 * @param ctx the parse tree
 */
fn enter_primaryKeyDefinition(&mut self, _ctx: &PrimaryKeyDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#primaryKeyDefinition}.
 * @param ctx the parse tree
 */
fn exit_primaryKeyDefinition(&mut self, _ctx: &PrimaryKeyDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#singlePrimaryKey}.
 * @param ctx the parse tree
 */
fn enter_singlePrimaryKey(&mut self, _ctx: &SinglePrimaryKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#singlePrimaryKey}.
 * @param ctx the parse tree
 */
fn exit_singlePrimaryKey(&mut self, _ctx: &SinglePrimaryKeyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#compoundKey}.
 * @param ctx the parse tree
 */
fn enter_compoundKey(&mut self, _ctx: &CompoundKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#compoundKey}.
 * @param ctx the parse tree
 */
fn exit_compoundKey(&mut self, _ctx: &CompoundKeyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#compositeKey}.
 * @param ctx the parse tree
 */
fn enter_compositeKey(&mut self, _ctx: &CompositeKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#compositeKey}.
 * @param ctx the parse tree
 */
fn exit_compositeKey(&mut self, _ctx: &CompositeKeyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#partitionKeyList}.
 * @param ctx the parse tree
 */
fn enter_partitionKeyList(&mut self, _ctx: &PartitionKeyListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#partitionKeyList}.
 * @param ctx the parse tree
 */
fn exit_partitionKeyList(&mut self, _ctx: &PartitionKeyListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#clusteringKeyList}.
 * @param ctx the parse tree
 */
fn enter_clusteringKeyList(&mut self, _ctx: &ClusteringKeyListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#clusteringKeyList}.
 * @param ctx the parse tree
 */
fn exit_clusteringKeyList(&mut self, _ctx: &ClusteringKeyListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#partitionKey}.
 * @param ctx the parse tree
 */
fn enter_partitionKey(&mut self, _ctx: &PartitionKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#partitionKey}.
 * @param ctx the parse tree
 */
fn exit_partitionKey(&mut self, _ctx: &PartitionKeyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#clusteringKey}.
 * @param ctx the parse tree
 */
fn enter_clusteringKey(&mut self, _ctx: &ClusteringKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#clusteringKey}.
 * @param ctx the parse tree
 */
fn exit_clusteringKey(&mut self, _ctx: &ClusteringKeyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#applyBatch}.
 * @param ctx the parse tree
 */
fn enter_applyBatch(&mut self, _ctx: &ApplyBatchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#applyBatch}.
 * @param ctx the parse tree
 */
fn exit_applyBatch(&mut self, _ctx: &ApplyBatchContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#beginBatch}.
 * @param ctx the parse tree
 */
fn enter_beginBatch(&mut self, _ctx: &BeginBatchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#beginBatch}.
 * @param ctx the parse tree
 */
fn exit_beginBatch(&mut self, _ctx: &BeginBatchContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#batchType}.
 * @param ctx the parse tree
 */
fn enter_batchType(&mut self, _ctx: &BatchTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#batchType}.
 * @param ctx the parse tree
 */
fn exit_batchType(&mut self, _ctx: &BatchTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#alterKeyspace}.
 * @param ctx the parse tree
 */
fn enter_alterKeyspace(&mut self, _ctx: &AlterKeyspaceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#alterKeyspace}.
 * @param ctx the parse tree
 */
fn exit_alterKeyspace(&mut self, _ctx: &AlterKeyspaceContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#replicationList}.
 * @param ctx the parse tree
 */
fn enter_replicationList(&mut self, _ctx: &ReplicationListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#replicationList}.
 * @param ctx the parse tree
 */
fn exit_replicationList(&mut self, _ctx: &ReplicationListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#replicationListItem}.
 * @param ctx the parse tree
 */
fn enter_replicationListItem(&mut self, _ctx: &ReplicationListItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#replicationListItem}.
 * @param ctx the parse tree
 */
fn exit_replicationListItem(&mut self, _ctx: &ReplicationListItemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#durableWrites}.
 * @param ctx the parse tree
 */
fn enter_durableWrites(&mut self, _ctx: &DurableWritesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#durableWrites}.
 * @param ctx the parse tree
 */
fn exit_durableWrites(&mut self, _ctx: &DurableWritesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#use_}.
 * @param ctx the parse tree
 */
fn enter_use_(&mut self, _ctx: &Use_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#use_}.
 * @param ctx the parse tree
 */
fn exit_use_(&mut self, _ctx: &Use_Context<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#truncate}.
 * @param ctx the parse tree
 */
fn enter_truncate(&mut self, _ctx: &TruncateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#truncate}.
 * @param ctx the parse tree
 */
fn exit_truncate(&mut self, _ctx: &TruncateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#createIndex}.
 * @param ctx the parse tree
 */
fn enter_createIndex(&mut self, _ctx: &CreateIndexContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#createIndex}.
 * @param ctx the parse tree
 */
fn exit_createIndex(&mut self, _ctx: &CreateIndexContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#indexName}.
 * @param ctx the parse tree
 */
fn enter_indexName(&mut self, _ctx: &IndexNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#indexName}.
 * @param ctx the parse tree
 */
fn exit_indexName(&mut self, _ctx: &IndexNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#indexColumnSpec}.
 * @param ctx the parse tree
 */
fn enter_indexColumnSpec(&mut self, _ctx: &IndexColumnSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#indexColumnSpec}.
 * @param ctx the parse tree
 */
fn exit_indexColumnSpec(&mut self, _ctx: &IndexColumnSpecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#indexKeysSpec}.
 * @param ctx the parse tree
 */
fn enter_indexKeysSpec(&mut self, _ctx: &IndexKeysSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#indexKeysSpec}.
 * @param ctx the parse tree
 */
fn exit_indexKeysSpec(&mut self, _ctx: &IndexKeysSpecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#indexEntriesSSpec}.
 * @param ctx the parse tree
 */
fn enter_indexEntriesSSpec(&mut self, _ctx: &IndexEntriesSSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#indexEntriesSSpec}.
 * @param ctx the parse tree
 */
fn exit_indexEntriesSSpec(&mut self, _ctx: &IndexEntriesSSpecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#indexFullSpec}.
 * @param ctx the parse tree
 */
fn enter_indexFullSpec(&mut self, _ctx: &IndexFullSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#indexFullSpec}.
 * @param ctx the parse tree
 */
fn exit_indexFullSpec(&mut self, _ctx: &IndexFullSpecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#delete_}.
 * @param ctx the parse tree
 */
fn enter_delete_(&mut self, _ctx: &Delete_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#delete_}.
 * @param ctx the parse tree
 */
fn exit_delete_(&mut self, _ctx: &Delete_Context<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#deleteColumnList}.
 * @param ctx the parse tree
 */
fn enter_deleteColumnList(&mut self, _ctx: &DeleteColumnListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#deleteColumnList}.
 * @param ctx the parse tree
 */
fn exit_deleteColumnList(&mut self, _ctx: &DeleteColumnListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#deleteColumnItem}.
 * @param ctx the parse tree
 */
fn enter_deleteColumnItem(&mut self, _ctx: &DeleteColumnItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#deleteColumnItem}.
 * @param ctx the parse tree
 */
fn exit_deleteColumnItem(&mut self, _ctx: &DeleteColumnItemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#update}.
 * @param ctx the parse tree
 */
fn enter_update(&mut self, _ctx: &UpdateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#update}.
 * @param ctx the parse tree
 */
fn exit_update(&mut self, _ctx: &UpdateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#ifSpec}.
 * @param ctx the parse tree
 */
fn enter_ifSpec(&mut self, _ctx: &IfSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#ifSpec}.
 * @param ctx the parse tree
 */
fn exit_ifSpec(&mut self, _ctx: &IfSpecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#ifConditionList}.
 * @param ctx the parse tree
 */
fn enter_ifConditionList(&mut self, _ctx: &IfConditionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#ifConditionList}.
 * @param ctx the parse tree
 */
fn exit_ifConditionList(&mut self, _ctx: &IfConditionListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#ifCondition}.
 * @param ctx the parse tree
 */
fn enter_ifCondition(&mut self, _ctx: &IfConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#ifCondition}.
 * @param ctx the parse tree
 */
fn exit_ifCondition(&mut self, _ctx: &IfConditionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#assignments}.
 * @param ctx the parse tree
 */
fn enter_assignments(&mut self, _ctx: &AssignmentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#assignments}.
 * @param ctx the parse tree
 */
fn exit_assignments(&mut self, _ctx: &AssignmentsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#assignmentElement}.
 * @param ctx the parse tree
 */
fn enter_assignmentElement(&mut self, _ctx: &AssignmentElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#assignmentElement}.
 * @param ctx the parse tree
 */
fn exit_assignmentElement(&mut self, _ctx: &AssignmentElementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#assignmentSet}.
 * @param ctx the parse tree
 */
fn enter_assignmentSet(&mut self, _ctx: &AssignmentSetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#assignmentSet}.
 * @param ctx the parse tree
 */
fn exit_assignmentSet(&mut self, _ctx: &AssignmentSetContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#assignmentMap}.
 * @param ctx the parse tree
 */
fn enter_assignmentMap(&mut self, _ctx: &AssignmentMapContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#assignmentMap}.
 * @param ctx the parse tree
 */
fn exit_assignmentMap(&mut self, _ctx: &AssignmentMapContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#assignmentList}.
 * @param ctx the parse tree
 */
fn enter_assignmentList(&mut self, _ctx: &AssignmentListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#assignmentList}.
 * @param ctx the parse tree
 */
fn exit_assignmentList(&mut self, _ctx: &AssignmentListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#assignmentTuple}.
 * @param ctx the parse tree
 */
fn enter_assignmentTuple(&mut self, _ctx: &AssignmentTupleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#assignmentTuple}.
 * @param ctx the parse tree
 */
fn exit_assignmentTuple(&mut self, _ctx: &AssignmentTupleContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#insert}.
 * @param ctx the parse tree
 */
fn enter_insert(&mut self, _ctx: &InsertContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#insert}.
 * @param ctx the parse tree
 */
fn exit_insert(&mut self, _ctx: &InsertContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#usingTtlTimestamp}.
 * @param ctx the parse tree
 */
fn enter_usingTtlTimestamp(&mut self, _ctx: &UsingTtlTimestampContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#usingTtlTimestamp}.
 * @param ctx the parse tree
 */
fn exit_usingTtlTimestamp(&mut self, _ctx: &UsingTtlTimestampContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#timestamp}.
 * @param ctx the parse tree
 */
fn enter_timestamp(&mut self, _ctx: &TimestampContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#timestamp}.
 * @param ctx the parse tree
 */
fn exit_timestamp(&mut self, _ctx: &TimestampContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#ttl}.
 * @param ctx the parse tree
 */
fn enter_ttl(&mut self, _ctx: &TtlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#ttl}.
 * @param ctx the parse tree
 */
fn exit_ttl(&mut self, _ctx: &TtlContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#usingTimestampSpec}.
 * @param ctx the parse tree
 */
fn enter_usingTimestampSpec(&mut self, _ctx: &UsingTimestampSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#usingTimestampSpec}.
 * @param ctx the parse tree
 */
fn exit_usingTimestampSpec(&mut self, _ctx: &UsingTimestampSpecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#ifNotExist}.
 * @param ctx the parse tree
 */
fn enter_ifNotExist(&mut self, _ctx: &IfNotExistContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#ifNotExist}.
 * @param ctx the parse tree
 */
fn exit_ifNotExist(&mut self, _ctx: &IfNotExistContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#ifExist}.
 * @param ctx the parse tree
 */
fn enter_ifExist(&mut self, _ctx: &IfExistContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#ifExist}.
 * @param ctx the parse tree
 */
fn exit_ifExist(&mut self, _ctx: &IfExistContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#insertValuesSpec}.
 * @param ctx the parse tree
 */
fn enter_insertValuesSpec(&mut self, _ctx: &InsertValuesSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#insertValuesSpec}.
 * @param ctx the parse tree
 */
fn exit_insertValuesSpec(&mut self, _ctx: &InsertValuesSpecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#insertColumnSpec}.
 * @param ctx the parse tree
 */
fn enter_insertColumnSpec(&mut self, _ctx: &InsertColumnSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#insertColumnSpec}.
 * @param ctx the parse tree
 */
fn exit_insertColumnSpec(&mut self, _ctx: &InsertColumnSpecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#columnList}.
 * @param ctx the parse tree
 */
fn enter_columnList(&mut self, _ctx: &ColumnListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#columnList}.
 * @param ctx the parse tree
 */
fn exit_columnList(&mut self, _ctx: &ColumnListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#expressionList}.
 * @param ctx the parse tree
 */
fn enter_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#expressionList}.
 * @param ctx the parse tree
 */
fn exit_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#select_}.
 * @param ctx the parse tree
 */
fn enter_select_(&mut self, _ctx: &Select_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#select_}.
 * @param ctx the parse tree
 */
fn exit_select_(&mut self, _ctx: &Select_Context<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#allowFilteringSpec}.
 * @param ctx the parse tree
 */
fn enter_allowFilteringSpec(&mut self, _ctx: &AllowFilteringSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#allowFilteringSpec}.
 * @param ctx the parse tree
 */
fn exit_allowFilteringSpec(&mut self, _ctx: &AllowFilteringSpecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#limitSpec}.
 * @param ctx the parse tree
 */
fn enter_limitSpec(&mut self, _ctx: &LimitSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#limitSpec}.
 * @param ctx the parse tree
 */
fn exit_limitSpec(&mut self, _ctx: &LimitSpecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#fromSpec}.
 * @param ctx the parse tree
 */
fn enter_fromSpec(&mut self, _ctx: &FromSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#fromSpec}.
 * @param ctx the parse tree
 */
fn exit_fromSpec(&mut self, _ctx: &FromSpecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#fromSpecElement}.
 * @param ctx the parse tree
 */
fn enter_fromSpecElement(&mut self, _ctx: &FromSpecElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#fromSpecElement}.
 * @param ctx the parse tree
 */
fn exit_fromSpecElement(&mut self, _ctx: &FromSpecElementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#orderSpec}.
 * @param ctx the parse tree
 */
fn enter_orderSpec(&mut self, _ctx: &OrderSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#orderSpec}.
 * @param ctx the parse tree
 */
fn exit_orderSpec(&mut self, _ctx: &OrderSpecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#orderSpecElement}.
 * @param ctx the parse tree
 */
fn enter_orderSpecElement(&mut self, _ctx: &OrderSpecElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#orderSpecElement}.
 * @param ctx the parse tree
 */
fn exit_orderSpecElement(&mut self, _ctx: &OrderSpecElementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#whereSpec}.
 * @param ctx the parse tree
 */
fn enter_whereSpec(&mut self, _ctx: &WhereSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#whereSpec}.
 * @param ctx the parse tree
 */
fn exit_whereSpec(&mut self, _ctx: &WhereSpecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#distinctSpec}.
 * @param ctx the parse tree
 */
fn enter_distinctSpec(&mut self, _ctx: &DistinctSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#distinctSpec}.
 * @param ctx the parse tree
 */
fn exit_distinctSpec(&mut self, _ctx: &DistinctSpecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#selectElements}.
 * @param ctx the parse tree
 */
fn enter_selectElements(&mut self, _ctx: &SelectElementsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#selectElements}.
 * @param ctx the parse tree
 */
fn exit_selectElements(&mut self, _ctx: &SelectElementsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#selectElement}.
 * @param ctx the parse tree
 */
fn enter_selectElement(&mut self, _ctx: &SelectElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#selectElement}.
 * @param ctx the parse tree
 */
fn exit_selectElement(&mut self, _ctx: &SelectElementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#relationElements}.
 * @param ctx the parse tree
 */
fn enter_relationElements(&mut self, _ctx: &RelationElementsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#relationElements}.
 * @param ctx the parse tree
 */
fn exit_relationElements(&mut self, _ctx: &RelationElementsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#relationElement}.
 * @param ctx the parse tree
 */
fn enter_relationElement(&mut self, _ctx: &RelationElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#relationElement}.
 * @param ctx the parse tree
 */
fn exit_relationElement(&mut self, _ctx: &RelationElementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#relalationContains}.
 * @param ctx the parse tree
 */
fn enter_relalationContains(&mut self, _ctx: &RelalationContainsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#relalationContains}.
 * @param ctx the parse tree
 */
fn exit_relalationContains(&mut self, _ctx: &RelalationContainsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#relalationContainsKey}.
 * @param ctx the parse tree
 */
fn enter_relalationContainsKey(&mut self, _ctx: &RelalationContainsKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#relalationContainsKey}.
 * @param ctx the parse tree
 */
fn exit_relalationContainsKey(&mut self, _ctx: &RelalationContainsKeyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#functionCall}.
 * @param ctx the parse tree
 */
fn enter_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#functionCall}.
 * @param ctx the parse tree
 */
fn exit_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#functionArgs}.
 * @param ctx the parse tree
 */
fn enter_functionArgs(&mut self, _ctx: &FunctionArgsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#functionArgs}.
 * @param ctx the parse tree
 */
fn exit_functionArgs(&mut self, _ctx: &FunctionArgsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#constant}.
 * @param ctx the parse tree
 */
fn enter_constant(&mut self, _ctx: &ConstantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#constant}.
 * @param ctx the parse tree
 */
fn exit_constant(&mut self, _ctx: &ConstantContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#decimalLiteral}.
 * @param ctx the parse tree
 */
fn enter_decimalLiteral(&mut self, _ctx: &DecimalLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#decimalLiteral}.
 * @param ctx the parse tree
 */
fn exit_decimalLiteral(&mut self, _ctx: &DecimalLiteralContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#floatLiteral}.
 * @param ctx the parse tree
 */
fn enter_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#floatLiteral}.
 * @param ctx the parse tree
 */
fn exit_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#stringLiteral}.
 * @param ctx the parse tree
 */
fn enter_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#stringLiteral}.
 * @param ctx the parse tree
 */
fn exit_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn enter_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn exit_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#hexadecimalLiteral}.
 * @param ctx the parse tree
 */
fn enter_hexadecimalLiteral(&mut self, _ctx: &HexadecimalLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#hexadecimalLiteral}.
 * @param ctx the parse tree
 */
fn exit_hexadecimalLiteral(&mut self, _ctx: &HexadecimalLiteralContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#keyspace}.
 * @param ctx the parse tree
 */
fn enter_keyspace(&mut self, _ctx: &KeyspaceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#keyspace}.
 * @param ctx the parse tree
 */
fn exit_keyspace(&mut self, _ctx: &KeyspaceContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#table}.
 * @param ctx the parse tree
 */
fn enter_table(&mut self, _ctx: &TableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#table}.
 * @param ctx the parse tree
 */
fn exit_table(&mut self, _ctx: &TableContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#column}.
 * @param ctx the parse tree
 */
fn enter_column(&mut self, _ctx: &ColumnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#column}.
 * @param ctx the parse tree
 */
fn exit_column(&mut self, _ctx: &ColumnContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#dataType}.
 * @param ctx the parse tree
 */
fn enter_dataType(&mut self, _ctx: &DataTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#dataType}.
 * @param ctx the parse tree
 */
fn exit_dataType(&mut self, _ctx: &DataTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#dataTypeName}.
 * @param ctx the parse tree
 */
fn enter_dataTypeName(&mut self, _ctx: &DataTypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#dataTypeName}.
 * @param ctx the parse tree
 */
fn exit_dataTypeName(&mut self, _ctx: &DataTypeNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#dataTypeDefinition}.
 * @param ctx the parse tree
 */
fn enter_dataTypeDefinition(&mut self, _ctx: &DataTypeDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#dataTypeDefinition}.
 * @param ctx the parse tree
 */
fn exit_dataTypeDefinition(&mut self, _ctx: &DataTypeDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#orderDirection}.
 * @param ctx the parse tree
 */
fn enter_orderDirection(&mut self, _ctx: &OrderDirectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#orderDirection}.
 * @param ctx the parse tree
 */
fn exit_orderDirection(&mut self, _ctx: &OrderDirectionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#role}.
 * @param ctx the parse tree
 */
fn enter_role(&mut self, _ctx: &RoleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#role}.
 * @param ctx the parse tree
 */
fn exit_role(&mut self, _ctx: &RoleContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#trigger}.
 * @param ctx the parse tree
 */
fn enter_trigger(&mut self, _ctx: &TriggerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#trigger}.
 * @param ctx the parse tree
 */
fn exit_trigger(&mut self, _ctx: &TriggerContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#triggerClass}.
 * @param ctx the parse tree
 */
fn enter_triggerClass(&mut self, _ctx: &TriggerClassContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#triggerClass}.
 * @param ctx the parse tree
 */
fn exit_triggerClass(&mut self, _ctx: &TriggerClassContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#materializedView}.
 * @param ctx the parse tree
 */
fn enter_materializedView(&mut self, _ctx: &MaterializedViewContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#materializedView}.
 * @param ctx the parse tree
 */
fn exit_materializedView(&mut self, _ctx: &MaterializedViewContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#type_}.
 * @param ctx the parse tree
 */
fn enter_type_(&mut self, _ctx: &Type_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#type_}.
 * @param ctx the parse tree
 */
fn exit_type_(&mut self, _ctx: &Type_Context<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#aggregate}.
 * @param ctx the parse tree
 */
fn enter_aggregate(&mut self, _ctx: &AggregateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#aggregate}.
 * @param ctx the parse tree
 */
fn exit_aggregate(&mut self, _ctx: &AggregateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#function_}.
 * @param ctx the parse tree
 */
fn enter_function_(&mut self, _ctx: &Function_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#function_}.
 * @param ctx the parse tree
 */
fn exit_function_(&mut self, _ctx: &Function_Context<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#language}.
 * @param ctx the parse tree
 */
fn enter_language(&mut self, _ctx: &LanguageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#language}.
 * @param ctx the parse tree
 */
fn exit_language(&mut self, _ctx: &LanguageContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#user}.
 * @param ctx the parse tree
 */
fn enter_user(&mut self, _ctx: &UserContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#user}.
 * @param ctx the parse tree
 */
fn exit_user(&mut self, _ctx: &UserContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#password}.
 * @param ctx the parse tree
 */
fn enter_password(&mut self, _ctx: &PasswordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#password}.
 * @param ctx the parse tree
 */
fn exit_password(&mut self, _ctx: &PasswordContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#hashKey}.
 * @param ctx the parse tree
 */
fn enter_hashKey(&mut self, _ctx: &HashKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#hashKey}.
 * @param ctx the parse tree
 */
fn exit_hashKey(&mut self, _ctx: &HashKeyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#param}.
 * @param ctx the parse tree
 */
fn enter_param(&mut self, _ctx: &ParamContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#param}.
 * @param ctx the parse tree
 */
fn exit_param(&mut self, _ctx: &ParamContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#paramName}.
 * @param ctx the parse tree
 */
fn enter_paramName(&mut self, _ctx: &ParamNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#paramName}.
 * @param ctx the parse tree
 */
fn exit_paramName(&mut self, _ctx: &ParamNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwAdd}.
 * @param ctx the parse tree
 */
fn enter_kwAdd(&mut self, _ctx: &KwAddContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwAdd}.
 * @param ctx the parse tree
 */
fn exit_kwAdd(&mut self, _ctx: &KwAddContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwAggregate}.
 * @param ctx the parse tree
 */
fn enter_kwAggregate(&mut self, _ctx: &KwAggregateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwAggregate}.
 * @param ctx the parse tree
 */
fn exit_kwAggregate(&mut self, _ctx: &KwAggregateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwAll}.
 * @param ctx the parse tree
 */
fn enter_kwAll(&mut self, _ctx: &KwAllContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwAll}.
 * @param ctx the parse tree
 */
fn exit_kwAll(&mut self, _ctx: &KwAllContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwAllPermissions}.
 * @param ctx the parse tree
 */
fn enter_kwAllPermissions(&mut self, _ctx: &KwAllPermissionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwAllPermissions}.
 * @param ctx the parse tree
 */
fn exit_kwAllPermissions(&mut self, _ctx: &KwAllPermissionsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwAllow}.
 * @param ctx the parse tree
 */
fn enter_kwAllow(&mut self, _ctx: &KwAllowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwAllow}.
 * @param ctx the parse tree
 */
fn exit_kwAllow(&mut self, _ctx: &KwAllowContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwAlter}.
 * @param ctx the parse tree
 */
fn enter_kwAlter(&mut self, _ctx: &KwAlterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwAlter}.
 * @param ctx the parse tree
 */
fn exit_kwAlter(&mut self, _ctx: &KwAlterContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwAnd}.
 * @param ctx the parse tree
 */
fn enter_kwAnd(&mut self, _ctx: &KwAndContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwAnd}.
 * @param ctx the parse tree
 */
fn exit_kwAnd(&mut self, _ctx: &KwAndContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwApply}.
 * @param ctx the parse tree
 */
fn enter_kwApply(&mut self, _ctx: &KwApplyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwApply}.
 * @param ctx the parse tree
 */
fn exit_kwApply(&mut self, _ctx: &KwApplyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwAs}.
 * @param ctx the parse tree
 */
fn enter_kwAs(&mut self, _ctx: &KwAsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwAs}.
 * @param ctx the parse tree
 */
fn exit_kwAs(&mut self, _ctx: &KwAsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwAsc}.
 * @param ctx the parse tree
 */
fn enter_kwAsc(&mut self, _ctx: &KwAscContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwAsc}.
 * @param ctx the parse tree
 */
fn exit_kwAsc(&mut self, _ctx: &KwAscContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwAuthorize}.
 * @param ctx the parse tree
 */
fn enter_kwAuthorize(&mut self, _ctx: &KwAuthorizeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwAuthorize}.
 * @param ctx the parse tree
 */
fn exit_kwAuthorize(&mut self, _ctx: &KwAuthorizeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwBatch}.
 * @param ctx the parse tree
 */
fn enter_kwBatch(&mut self, _ctx: &KwBatchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwBatch}.
 * @param ctx the parse tree
 */
fn exit_kwBatch(&mut self, _ctx: &KwBatchContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwBegin}.
 * @param ctx the parse tree
 */
fn enter_kwBegin(&mut self, _ctx: &KwBeginContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwBegin}.
 * @param ctx the parse tree
 */
fn exit_kwBegin(&mut self, _ctx: &KwBeginContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwBy}.
 * @param ctx the parse tree
 */
fn enter_kwBy(&mut self, _ctx: &KwByContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwBy}.
 * @param ctx the parse tree
 */
fn exit_kwBy(&mut self, _ctx: &KwByContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwCalled}.
 * @param ctx the parse tree
 */
fn enter_kwCalled(&mut self, _ctx: &KwCalledContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwCalled}.
 * @param ctx the parse tree
 */
fn exit_kwCalled(&mut self, _ctx: &KwCalledContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwClustering}.
 * @param ctx the parse tree
 */
fn enter_kwClustering(&mut self, _ctx: &KwClusteringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwClustering}.
 * @param ctx the parse tree
 */
fn exit_kwClustering(&mut self, _ctx: &KwClusteringContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwCompact}.
 * @param ctx the parse tree
 */
fn enter_kwCompact(&mut self, _ctx: &KwCompactContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwCompact}.
 * @param ctx the parse tree
 */
fn exit_kwCompact(&mut self, _ctx: &KwCompactContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwContains}.
 * @param ctx the parse tree
 */
fn enter_kwContains(&mut self, _ctx: &KwContainsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwContains}.
 * @param ctx the parse tree
 */
fn exit_kwContains(&mut self, _ctx: &KwContainsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwCreate}.
 * @param ctx the parse tree
 */
fn enter_kwCreate(&mut self, _ctx: &KwCreateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwCreate}.
 * @param ctx the parse tree
 */
fn exit_kwCreate(&mut self, _ctx: &KwCreateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwDelete}.
 * @param ctx the parse tree
 */
fn enter_kwDelete(&mut self, _ctx: &KwDeleteContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwDelete}.
 * @param ctx the parse tree
 */
fn exit_kwDelete(&mut self, _ctx: &KwDeleteContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwDesc}.
 * @param ctx the parse tree
 */
fn enter_kwDesc(&mut self, _ctx: &KwDescContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwDesc}.
 * @param ctx the parse tree
 */
fn exit_kwDesc(&mut self, _ctx: &KwDescContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwDescibe}.
 * @param ctx the parse tree
 */
fn enter_kwDescibe(&mut self, _ctx: &KwDescibeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwDescibe}.
 * @param ctx the parse tree
 */
fn exit_kwDescibe(&mut self, _ctx: &KwDescibeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwDistinct}.
 * @param ctx the parse tree
 */
fn enter_kwDistinct(&mut self, _ctx: &KwDistinctContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwDistinct}.
 * @param ctx the parse tree
 */
fn exit_kwDistinct(&mut self, _ctx: &KwDistinctContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwDrop}.
 * @param ctx the parse tree
 */
fn enter_kwDrop(&mut self, _ctx: &KwDropContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwDrop}.
 * @param ctx the parse tree
 */
fn exit_kwDrop(&mut self, _ctx: &KwDropContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwDurableWrites}.
 * @param ctx the parse tree
 */
fn enter_kwDurableWrites(&mut self, _ctx: &KwDurableWritesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwDurableWrites}.
 * @param ctx the parse tree
 */
fn exit_kwDurableWrites(&mut self, _ctx: &KwDurableWritesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwEntries}.
 * @param ctx the parse tree
 */
fn enter_kwEntries(&mut self, _ctx: &KwEntriesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwEntries}.
 * @param ctx the parse tree
 */
fn exit_kwEntries(&mut self, _ctx: &KwEntriesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwExecute}.
 * @param ctx the parse tree
 */
fn enter_kwExecute(&mut self, _ctx: &KwExecuteContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwExecute}.
 * @param ctx the parse tree
 */
fn exit_kwExecute(&mut self, _ctx: &KwExecuteContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwExists}.
 * @param ctx the parse tree
 */
fn enter_kwExists(&mut self, _ctx: &KwExistsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwExists}.
 * @param ctx the parse tree
 */
fn exit_kwExists(&mut self, _ctx: &KwExistsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwFiltering}.
 * @param ctx the parse tree
 */
fn enter_kwFiltering(&mut self, _ctx: &KwFilteringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwFiltering}.
 * @param ctx the parse tree
 */
fn exit_kwFiltering(&mut self, _ctx: &KwFilteringContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwFinalfunc}.
 * @param ctx the parse tree
 */
fn enter_kwFinalfunc(&mut self, _ctx: &KwFinalfuncContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwFinalfunc}.
 * @param ctx the parse tree
 */
fn exit_kwFinalfunc(&mut self, _ctx: &KwFinalfuncContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwFrom}.
 * @param ctx the parse tree
 */
fn enter_kwFrom(&mut self, _ctx: &KwFromContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwFrom}.
 * @param ctx the parse tree
 */
fn exit_kwFrom(&mut self, _ctx: &KwFromContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwFull}.
 * @param ctx the parse tree
 */
fn enter_kwFull(&mut self, _ctx: &KwFullContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwFull}.
 * @param ctx the parse tree
 */
fn exit_kwFull(&mut self, _ctx: &KwFullContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwFunction}.
 * @param ctx the parse tree
 */
fn enter_kwFunction(&mut self, _ctx: &KwFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwFunction}.
 * @param ctx the parse tree
 */
fn exit_kwFunction(&mut self, _ctx: &KwFunctionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwFunctions}.
 * @param ctx the parse tree
 */
fn enter_kwFunctions(&mut self, _ctx: &KwFunctionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwFunctions}.
 * @param ctx the parse tree
 */
fn exit_kwFunctions(&mut self, _ctx: &KwFunctionsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwGrant}.
 * @param ctx the parse tree
 */
fn enter_kwGrant(&mut self, _ctx: &KwGrantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwGrant}.
 * @param ctx the parse tree
 */
fn exit_kwGrant(&mut self, _ctx: &KwGrantContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwIf}.
 * @param ctx the parse tree
 */
fn enter_kwIf(&mut self, _ctx: &KwIfContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwIf}.
 * @param ctx the parse tree
 */
fn exit_kwIf(&mut self, _ctx: &KwIfContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwIn}.
 * @param ctx the parse tree
 */
fn enter_kwIn(&mut self, _ctx: &KwInContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwIn}.
 * @param ctx the parse tree
 */
fn exit_kwIn(&mut self, _ctx: &KwInContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwIndex}.
 * @param ctx the parse tree
 */
fn enter_kwIndex(&mut self, _ctx: &KwIndexContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwIndex}.
 * @param ctx the parse tree
 */
fn exit_kwIndex(&mut self, _ctx: &KwIndexContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwInitcond}.
 * @param ctx the parse tree
 */
fn enter_kwInitcond(&mut self, _ctx: &KwInitcondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwInitcond}.
 * @param ctx the parse tree
 */
fn exit_kwInitcond(&mut self, _ctx: &KwInitcondContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwInput}.
 * @param ctx the parse tree
 */
fn enter_kwInput(&mut self, _ctx: &KwInputContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwInput}.
 * @param ctx the parse tree
 */
fn exit_kwInput(&mut self, _ctx: &KwInputContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwInsert}.
 * @param ctx the parse tree
 */
fn enter_kwInsert(&mut self, _ctx: &KwInsertContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwInsert}.
 * @param ctx the parse tree
 */
fn exit_kwInsert(&mut self, _ctx: &KwInsertContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwInto}.
 * @param ctx the parse tree
 */
fn enter_kwInto(&mut self, _ctx: &KwIntoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwInto}.
 * @param ctx the parse tree
 */
fn exit_kwInto(&mut self, _ctx: &KwIntoContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwIs}.
 * @param ctx the parse tree
 */
fn enter_kwIs(&mut self, _ctx: &KwIsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwIs}.
 * @param ctx the parse tree
 */
fn exit_kwIs(&mut self, _ctx: &KwIsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwJson}.
 * @param ctx the parse tree
 */
fn enter_kwJson(&mut self, _ctx: &KwJsonContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwJson}.
 * @param ctx the parse tree
 */
fn exit_kwJson(&mut self, _ctx: &KwJsonContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwKey}.
 * @param ctx the parse tree
 */
fn enter_kwKey(&mut self, _ctx: &KwKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwKey}.
 * @param ctx the parse tree
 */
fn exit_kwKey(&mut self, _ctx: &KwKeyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwKeys}.
 * @param ctx the parse tree
 */
fn enter_kwKeys(&mut self, _ctx: &KwKeysContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwKeys}.
 * @param ctx the parse tree
 */
fn exit_kwKeys(&mut self, _ctx: &KwKeysContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwKeyspace}.
 * @param ctx the parse tree
 */
fn enter_kwKeyspace(&mut self, _ctx: &KwKeyspaceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwKeyspace}.
 * @param ctx the parse tree
 */
fn exit_kwKeyspace(&mut self, _ctx: &KwKeyspaceContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwKeyspaces}.
 * @param ctx the parse tree
 */
fn enter_kwKeyspaces(&mut self, _ctx: &KwKeyspacesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwKeyspaces}.
 * @param ctx the parse tree
 */
fn exit_kwKeyspaces(&mut self, _ctx: &KwKeyspacesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwLanguage}.
 * @param ctx the parse tree
 */
fn enter_kwLanguage(&mut self, _ctx: &KwLanguageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwLanguage}.
 * @param ctx the parse tree
 */
fn exit_kwLanguage(&mut self, _ctx: &KwLanguageContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwLimit}.
 * @param ctx the parse tree
 */
fn enter_kwLimit(&mut self, _ctx: &KwLimitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwLimit}.
 * @param ctx the parse tree
 */
fn exit_kwLimit(&mut self, _ctx: &KwLimitContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwList}.
 * @param ctx the parse tree
 */
fn enter_kwList(&mut self, _ctx: &KwListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwList}.
 * @param ctx the parse tree
 */
fn exit_kwList(&mut self, _ctx: &KwListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwLogged}.
 * @param ctx the parse tree
 */
fn enter_kwLogged(&mut self, _ctx: &KwLoggedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwLogged}.
 * @param ctx the parse tree
 */
fn exit_kwLogged(&mut self, _ctx: &KwLoggedContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwLogin}.
 * @param ctx the parse tree
 */
fn enter_kwLogin(&mut self, _ctx: &KwLoginContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwLogin}.
 * @param ctx the parse tree
 */
fn exit_kwLogin(&mut self, _ctx: &KwLoginContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwMaterialized}.
 * @param ctx the parse tree
 */
fn enter_kwMaterialized(&mut self, _ctx: &KwMaterializedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwMaterialized}.
 * @param ctx the parse tree
 */
fn exit_kwMaterialized(&mut self, _ctx: &KwMaterializedContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwModify}.
 * @param ctx the parse tree
 */
fn enter_kwModify(&mut self, _ctx: &KwModifyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwModify}.
 * @param ctx the parse tree
 */
fn exit_kwModify(&mut self, _ctx: &KwModifyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwNosuperuser}.
 * @param ctx the parse tree
 */
fn enter_kwNosuperuser(&mut self, _ctx: &KwNosuperuserContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwNosuperuser}.
 * @param ctx the parse tree
 */
fn exit_kwNosuperuser(&mut self, _ctx: &KwNosuperuserContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwNorecursive}.
 * @param ctx the parse tree
 */
fn enter_kwNorecursive(&mut self, _ctx: &KwNorecursiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwNorecursive}.
 * @param ctx the parse tree
 */
fn exit_kwNorecursive(&mut self, _ctx: &KwNorecursiveContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwNot}.
 * @param ctx the parse tree
 */
fn enter_kwNot(&mut self, _ctx: &KwNotContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwNot}.
 * @param ctx the parse tree
 */
fn exit_kwNot(&mut self, _ctx: &KwNotContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwNull}.
 * @param ctx the parse tree
 */
fn enter_kwNull(&mut self, _ctx: &KwNullContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwNull}.
 * @param ctx the parse tree
 */
fn exit_kwNull(&mut self, _ctx: &KwNullContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwOf}.
 * @param ctx the parse tree
 */
fn enter_kwOf(&mut self, _ctx: &KwOfContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwOf}.
 * @param ctx the parse tree
 */
fn exit_kwOf(&mut self, _ctx: &KwOfContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwOn}.
 * @param ctx the parse tree
 */
fn enter_kwOn(&mut self, _ctx: &KwOnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwOn}.
 * @param ctx the parse tree
 */
fn exit_kwOn(&mut self, _ctx: &KwOnContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwOptions}.
 * @param ctx the parse tree
 */
fn enter_kwOptions(&mut self, _ctx: &KwOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwOptions}.
 * @param ctx the parse tree
 */
fn exit_kwOptions(&mut self, _ctx: &KwOptionsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwOr}.
 * @param ctx the parse tree
 */
fn enter_kwOr(&mut self, _ctx: &KwOrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwOr}.
 * @param ctx the parse tree
 */
fn exit_kwOr(&mut self, _ctx: &KwOrContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwOrder}.
 * @param ctx the parse tree
 */
fn enter_kwOrder(&mut self, _ctx: &KwOrderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwOrder}.
 * @param ctx the parse tree
 */
fn exit_kwOrder(&mut self, _ctx: &KwOrderContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwPassword}.
 * @param ctx the parse tree
 */
fn enter_kwPassword(&mut self, _ctx: &KwPasswordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwPassword}.
 * @param ctx the parse tree
 */
fn exit_kwPassword(&mut self, _ctx: &KwPasswordContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwPrimary}.
 * @param ctx the parse tree
 */
fn enter_kwPrimary(&mut self, _ctx: &KwPrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwPrimary}.
 * @param ctx the parse tree
 */
fn exit_kwPrimary(&mut self, _ctx: &KwPrimaryContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwRename}.
 * @param ctx the parse tree
 */
fn enter_kwRename(&mut self, _ctx: &KwRenameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwRename}.
 * @param ctx the parse tree
 */
fn exit_kwRename(&mut self, _ctx: &KwRenameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwReplace}.
 * @param ctx the parse tree
 */
fn enter_kwReplace(&mut self, _ctx: &KwReplaceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwReplace}.
 * @param ctx the parse tree
 */
fn exit_kwReplace(&mut self, _ctx: &KwReplaceContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwReplication}.
 * @param ctx the parse tree
 */
fn enter_kwReplication(&mut self, _ctx: &KwReplicationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwReplication}.
 * @param ctx the parse tree
 */
fn exit_kwReplication(&mut self, _ctx: &KwReplicationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwReturns}.
 * @param ctx the parse tree
 */
fn enter_kwReturns(&mut self, _ctx: &KwReturnsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwReturns}.
 * @param ctx the parse tree
 */
fn exit_kwReturns(&mut self, _ctx: &KwReturnsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwRole}.
 * @param ctx the parse tree
 */
fn enter_kwRole(&mut self, _ctx: &KwRoleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwRole}.
 * @param ctx the parse tree
 */
fn exit_kwRole(&mut self, _ctx: &KwRoleContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwRoles}.
 * @param ctx the parse tree
 */
fn enter_kwRoles(&mut self, _ctx: &KwRolesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwRoles}.
 * @param ctx the parse tree
 */
fn exit_kwRoles(&mut self, _ctx: &KwRolesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwSelect}.
 * @param ctx the parse tree
 */
fn enter_kwSelect(&mut self, _ctx: &KwSelectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwSelect}.
 * @param ctx the parse tree
 */
fn exit_kwSelect(&mut self, _ctx: &KwSelectContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwSet}.
 * @param ctx the parse tree
 */
fn enter_kwSet(&mut self, _ctx: &KwSetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwSet}.
 * @param ctx the parse tree
 */
fn exit_kwSet(&mut self, _ctx: &KwSetContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwSfunc}.
 * @param ctx the parse tree
 */
fn enter_kwSfunc(&mut self, _ctx: &KwSfuncContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwSfunc}.
 * @param ctx the parse tree
 */
fn exit_kwSfunc(&mut self, _ctx: &KwSfuncContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwStorage}.
 * @param ctx the parse tree
 */
fn enter_kwStorage(&mut self, _ctx: &KwStorageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwStorage}.
 * @param ctx the parse tree
 */
fn exit_kwStorage(&mut self, _ctx: &KwStorageContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwStype}.
 * @param ctx the parse tree
 */
fn enter_kwStype(&mut self, _ctx: &KwStypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwStype}.
 * @param ctx the parse tree
 */
fn exit_kwStype(&mut self, _ctx: &KwStypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwSuperuser}.
 * @param ctx the parse tree
 */
fn enter_kwSuperuser(&mut self, _ctx: &KwSuperuserContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwSuperuser}.
 * @param ctx the parse tree
 */
fn exit_kwSuperuser(&mut self, _ctx: &KwSuperuserContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwTable}.
 * @param ctx the parse tree
 */
fn enter_kwTable(&mut self, _ctx: &KwTableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwTable}.
 * @param ctx the parse tree
 */
fn exit_kwTable(&mut self, _ctx: &KwTableContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwTimestamp}.
 * @param ctx the parse tree
 */
fn enter_kwTimestamp(&mut self, _ctx: &KwTimestampContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwTimestamp}.
 * @param ctx the parse tree
 */
fn exit_kwTimestamp(&mut self, _ctx: &KwTimestampContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwTo}.
 * @param ctx the parse tree
 */
fn enter_kwTo(&mut self, _ctx: &KwToContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwTo}.
 * @param ctx the parse tree
 */
fn exit_kwTo(&mut self, _ctx: &KwToContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwTrigger}.
 * @param ctx the parse tree
 */
fn enter_kwTrigger(&mut self, _ctx: &KwTriggerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwTrigger}.
 * @param ctx the parse tree
 */
fn exit_kwTrigger(&mut self, _ctx: &KwTriggerContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwTruncate}.
 * @param ctx the parse tree
 */
fn enter_kwTruncate(&mut self, _ctx: &KwTruncateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwTruncate}.
 * @param ctx the parse tree
 */
fn exit_kwTruncate(&mut self, _ctx: &KwTruncateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwTtl}.
 * @param ctx the parse tree
 */
fn enter_kwTtl(&mut self, _ctx: &KwTtlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwTtl}.
 * @param ctx the parse tree
 */
fn exit_kwTtl(&mut self, _ctx: &KwTtlContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwType}.
 * @param ctx the parse tree
 */
fn enter_kwType(&mut self, _ctx: &KwTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwType}.
 * @param ctx the parse tree
 */
fn exit_kwType(&mut self, _ctx: &KwTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwUnlogged}.
 * @param ctx the parse tree
 */
fn enter_kwUnlogged(&mut self, _ctx: &KwUnloggedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwUnlogged}.
 * @param ctx the parse tree
 */
fn exit_kwUnlogged(&mut self, _ctx: &KwUnloggedContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwUpdate}.
 * @param ctx the parse tree
 */
fn enter_kwUpdate(&mut self, _ctx: &KwUpdateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwUpdate}.
 * @param ctx the parse tree
 */
fn exit_kwUpdate(&mut self, _ctx: &KwUpdateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwUse}.
 * @param ctx the parse tree
 */
fn enter_kwUse(&mut self, _ctx: &KwUseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwUse}.
 * @param ctx the parse tree
 */
fn exit_kwUse(&mut self, _ctx: &KwUseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwUser}.
 * @param ctx the parse tree
 */
fn enter_kwUser(&mut self, _ctx: &KwUserContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwUser}.
 * @param ctx the parse tree
 */
fn exit_kwUser(&mut self, _ctx: &KwUserContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwUsers}.
 * @param ctx the parse tree
 */
fn enter_kwUsers(&mut self, _ctx: &KwUsersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwUsers}.
 * @param ctx the parse tree
 */
fn exit_kwUsers(&mut self, _ctx: &KwUsersContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwUsing}.
 * @param ctx the parse tree
 */
fn enter_kwUsing(&mut self, _ctx: &KwUsingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwUsing}.
 * @param ctx the parse tree
 */
fn exit_kwUsing(&mut self, _ctx: &KwUsingContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwValues}.
 * @param ctx the parse tree
 */
fn enter_kwValues(&mut self, _ctx: &KwValuesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwValues}.
 * @param ctx the parse tree
 */
fn exit_kwValues(&mut self, _ctx: &KwValuesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwView}.
 * @param ctx the parse tree
 */
fn enter_kwView(&mut self, _ctx: &KwViewContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwView}.
 * @param ctx the parse tree
 */
fn exit_kwView(&mut self, _ctx: &KwViewContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwWhere}.
 * @param ctx the parse tree
 */
fn enter_kwWhere(&mut self, _ctx: &KwWhereContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwWhere}.
 * @param ctx the parse tree
 */
fn exit_kwWhere(&mut self, _ctx: &KwWhereContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwWith}.
 * @param ctx the parse tree
 */
fn enter_kwWith(&mut self, _ctx: &KwWithContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwWith}.
 * @param ctx the parse tree
 */
fn exit_kwWith(&mut self, _ctx: &KwWithContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#kwRevoke}.
 * @param ctx the parse tree
 */
fn enter_kwRevoke(&mut self, _ctx: &KwRevokeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#kwRevoke}.
 * @param ctx the parse tree
 */
fn exit_kwRevoke(&mut self, _ctx: &KwRevokeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#eof}.
 * @param ctx the parse tree
 */
fn enter_eof(&mut self, _ctx: &EofContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#eof}.
 * @param ctx the parse tree
 */
fn exit_eof(&mut self, _ctx: &EofContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#syntaxBracketLr}.
 * @param ctx the parse tree
 */
fn enter_syntaxBracketLr(&mut self, _ctx: &SyntaxBracketLrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#syntaxBracketLr}.
 * @param ctx the parse tree
 */
fn exit_syntaxBracketLr(&mut self, _ctx: &SyntaxBracketLrContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#syntaxBracketRr}.
 * @param ctx the parse tree
 */
fn enter_syntaxBracketRr(&mut self, _ctx: &SyntaxBracketRrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#syntaxBracketRr}.
 * @param ctx the parse tree
 */
fn exit_syntaxBracketRr(&mut self, _ctx: &SyntaxBracketRrContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#syntaxBracketLc}.
 * @param ctx the parse tree
 */
fn enter_syntaxBracketLc(&mut self, _ctx: &SyntaxBracketLcContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#syntaxBracketLc}.
 * @param ctx the parse tree
 */
fn exit_syntaxBracketLc(&mut self, _ctx: &SyntaxBracketLcContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#syntaxBracketRc}.
 * @param ctx the parse tree
 */
fn enter_syntaxBracketRc(&mut self, _ctx: &SyntaxBracketRcContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#syntaxBracketRc}.
 * @param ctx the parse tree
 */
fn exit_syntaxBracketRc(&mut self, _ctx: &SyntaxBracketRcContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#syntaxBracketLa}.
 * @param ctx the parse tree
 */
fn enter_syntaxBracketLa(&mut self, _ctx: &SyntaxBracketLaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#syntaxBracketLa}.
 * @param ctx the parse tree
 */
fn exit_syntaxBracketLa(&mut self, _ctx: &SyntaxBracketLaContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#syntaxBracketRa}.
 * @param ctx the parse tree
 */
fn enter_syntaxBracketRa(&mut self, _ctx: &SyntaxBracketRaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#syntaxBracketRa}.
 * @param ctx the parse tree
 */
fn exit_syntaxBracketRa(&mut self, _ctx: &SyntaxBracketRaContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#syntaxBracketLs}.
 * @param ctx the parse tree
 */
fn enter_syntaxBracketLs(&mut self, _ctx: &SyntaxBracketLsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#syntaxBracketLs}.
 * @param ctx the parse tree
 */
fn exit_syntaxBracketLs(&mut self, _ctx: &SyntaxBracketLsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#syntaxBracketRs}.
 * @param ctx the parse tree
 */
fn enter_syntaxBracketRs(&mut self, _ctx: &SyntaxBracketRsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#syntaxBracketRs}.
 * @param ctx the parse tree
 */
fn exit_syntaxBracketRs(&mut self, _ctx: &SyntaxBracketRsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#syntaxComma}.
 * @param ctx the parse tree
 */
fn enter_syntaxComma(&mut self, _ctx: &SyntaxCommaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#syntaxComma}.
 * @param ctx the parse tree
 */
fn exit_syntaxComma(&mut self, _ctx: &SyntaxCommaContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CqlParser#syntaxColon}.
 * @param ctx the parse tree
 */
fn enter_syntaxColon(&mut self, _ctx: &SyntaxColonContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CqlParser#syntaxColon}.
 * @param ctx the parse tree
 */
fn exit_syntaxColon(&mut self, _ctx: &SyntaxColonContext<'input>) { }

}
