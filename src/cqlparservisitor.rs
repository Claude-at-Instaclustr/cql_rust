#![allow(nonstandard_style)]
// Generated from grammars-v4/cql3/CqlParser.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor};
use super::cqlparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link CqlParser}.
 */
pub trait CqlParserVisitor<'input>: ParseTreeVisitor<'input,CqlParserContextType>{
	/**
	 * Visit a parse tree produced by {@link CqlParser#root}.
	 * @param ctx the parse tree
	 */
	fn visit_root(&mut self, ctx: &RootContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#cqls}.
	 * @param ctx the parse tree
	 */
	fn visit_cqls(&mut self, ctx: &CqlsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#statementSeparator}.
	 * @param ctx the parse tree
	 */
	fn visit_statementSeparator(&mut self, ctx: &StatementSeparatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#empty_}.
	 * @param ctx the parse tree
	 */
	fn visit_empty_(&mut self, ctx: &Empty_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#cql}.
	 * @param ctx the parse tree
	 */
	fn visit_cql(&mut self, ctx: &CqlContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#revoke}.
	 * @param ctx the parse tree
	 */
	fn visit_revoke(&mut self, ctx: &RevokeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#listUsers}.
	 * @param ctx the parse tree
	 */
	fn visit_listUsers(&mut self, ctx: &ListUsersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#listRoles}.
	 * @param ctx the parse tree
	 */
	fn visit_listRoles(&mut self, ctx: &ListRolesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#listPermissions}.
	 * @param ctx the parse tree
	 */
	fn visit_listPermissions(&mut self, ctx: &ListPermissionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#grant}.
	 * @param ctx the parse tree
	 */
	fn visit_grant(&mut self, ctx: &GrantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#priviledge}.
	 * @param ctx the parse tree
	 */
	fn visit_priviledge(&mut self, ctx: &PriviledgeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#resource}.
	 * @param ctx the parse tree
	 */
	fn visit_resource(&mut self, ctx: &ResourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#createUser}.
	 * @param ctx the parse tree
	 */
	fn visit_createUser(&mut self, ctx: &CreateUserContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#createRole}.
	 * @param ctx the parse tree
	 */
	fn visit_createRole(&mut self, ctx: &CreateRoleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#createType}.
	 * @param ctx the parse tree
	 */
	fn visit_createType(&mut self, ctx: &CreateTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#typeMemberColumnList}.
	 * @param ctx the parse tree
	 */
	fn visit_typeMemberColumnList(&mut self, ctx: &TypeMemberColumnListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#createTrigger}.
	 * @param ctx the parse tree
	 */
	fn visit_createTrigger(&mut self, ctx: &CreateTriggerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#createMaterializedView}.
	 * @param ctx the parse tree
	 */
	fn visit_createMaterializedView(&mut self, ctx: &CreateMaterializedViewContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#materializedViewWhere}.
	 * @param ctx the parse tree
	 */
	fn visit_materializedViewWhere(&mut self, ctx: &MaterializedViewWhereContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#columnNotNullList}.
	 * @param ctx the parse tree
	 */
	fn visit_columnNotNullList(&mut self, ctx: &ColumnNotNullListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#columnNotNull}.
	 * @param ctx the parse tree
	 */
	fn visit_columnNotNull(&mut self, ctx: &ColumnNotNullContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#materializedViewOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_materializedViewOptions(&mut self, ctx: &MaterializedViewOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#createKeyspace}.
	 * @param ctx the parse tree
	 */
	fn visit_createKeyspace(&mut self, ctx: &CreateKeyspaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#createFunction}.
	 * @param ctx the parse tree
	 */
	fn visit_createFunction(&mut self, ctx: &CreateFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#codeBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_codeBlock(&mut self, ctx: &CodeBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#paramList}.
	 * @param ctx the parse tree
	 */
	fn visit_paramList(&mut self, ctx: &ParamListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#returnMode}.
	 * @param ctx the parse tree
	 */
	fn visit_returnMode(&mut self, ctx: &ReturnModeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#createAggregate}.
	 * @param ctx the parse tree
	 */
	fn visit_createAggregate(&mut self, ctx: &CreateAggregateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#initCondDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_initCondDefinition(&mut self, ctx: &InitCondDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#initCondHash}.
	 * @param ctx the parse tree
	 */
	fn visit_initCondHash(&mut self, ctx: &InitCondHashContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#initCondHashItem}.
	 * @param ctx the parse tree
	 */
	fn visit_initCondHashItem(&mut self, ctx: &InitCondHashItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#initCondListNested}.
	 * @param ctx the parse tree
	 */
	fn visit_initCondListNested(&mut self, ctx: &InitCondListNestedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#initCondList}.
	 * @param ctx the parse tree
	 */
	fn visit_initCondList(&mut self, ctx: &InitCondListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#orReplace}.
	 * @param ctx the parse tree
	 */
	fn visit_orReplace(&mut self, ctx: &OrReplaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterUser}.
	 * @param ctx the parse tree
	 */
	fn visit_alterUser(&mut self, ctx: &AlterUserContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#userPassword}.
	 * @param ctx the parse tree
	 */
	fn visit_userPassword(&mut self, ctx: &UserPasswordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#userSuperUser}.
	 * @param ctx the parse tree
	 */
	fn visit_userSuperUser(&mut self, ctx: &UserSuperUserContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterType}.
	 * @param ctx the parse tree
	 */
	fn visit_alterType(&mut self, ctx: &AlterTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTypeOperation}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTypeOperation(&mut self, ctx: &AlterTypeOperationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTypeRename}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTypeRename(&mut self, ctx: &AlterTypeRenameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTypeRenameList}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTypeRenameList(&mut self, ctx: &AlterTypeRenameListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTypeRenameItem}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTypeRenameItem(&mut self, ctx: &AlterTypeRenameItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTypeAdd}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTypeAdd(&mut self, ctx: &AlterTypeAddContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTypeAlterType}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTypeAlterType(&mut self, ctx: &AlterTypeAlterTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTable}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTable(&mut self, ctx: &AlterTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTableOperation}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTableOperation(&mut self, ctx: &AlterTableOperationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTableWith}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTableWith(&mut self, ctx: &AlterTableWithContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTableRename}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTableRename(&mut self, ctx: &AlterTableRenameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTableDropCompactStorage}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTableDropCompactStorage(&mut self, ctx: &AlterTableDropCompactStorageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTableDropColumns}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTableDropColumns(&mut self, ctx: &AlterTableDropColumnsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTableDropColumnList}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTableDropColumnList(&mut self, ctx: &AlterTableDropColumnListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTableAdd}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTableAdd(&mut self, ctx: &AlterTableAddContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterTableColumnDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTableColumnDefinition(&mut self, ctx: &AlterTableColumnDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterRole}.
	 * @param ctx the parse tree
	 */
	fn visit_alterRole(&mut self, ctx: &AlterRoleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#roleWith}.
	 * @param ctx the parse tree
	 */
	fn visit_roleWith(&mut self, ctx: &RoleWithContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#roleWithOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_roleWithOptions(&mut self, ctx: &RoleWithOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterMaterializedView}.
	 * @param ctx the parse tree
	 */
	fn visit_alterMaterializedView(&mut self, ctx: &AlterMaterializedViewContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#dropUser}.
	 * @param ctx the parse tree
	 */
	fn visit_dropUser(&mut self, ctx: &DropUserContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#dropType}.
	 * @param ctx the parse tree
	 */
	fn visit_dropType(&mut self, ctx: &DropTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#dropMaterializedView}.
	 * @param ctx the parse tree
	 */
	fn visit_dropMaterializedView(&mut self, ctx: &DropMaterializedViewContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#dropAggregate}.
	 * @param ctx the parse tree
	 */
	fn visit_dropAggregate(&mut self, ctx: &DropAggregateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#dropFunction}.
	 * @param ctx the parse tree
	 */
	fn visit_dropFunction(&mut self, ctx: &DropFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#dropTrigger}.
	 * @param ctx the parse tree
	 */
	fn visit_dropTrigger(&mut self, ctx: &DropTriggerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#dropRole}.
	 * @param ctx the parse tree
	 */
	fn visit_dropRole(&mut self, ctx: &DropRoleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#dropTable}.
	 * @param ctx the parse tree
	 */
	fn visit_dropTable(&mut self, ctx: &DropTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#dropKeyspace}.
	 * @param ctx the parse tree
	 */
	fn visit_dropKeyspace(&mut self, ctx: &DropKeyspaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#dropIndex}.
	 * @param ctx the parse tree
	 */
	fn visit_dropIndex(&mut self, ctx: &DropIndexContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#createTable}.
	 * @param ctx the parse tree
	 */
	fn visit_createTable(&mut self, ctx: &CreateTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#withElement}.
	 * @param ctx the parse tree
	 */
	fn visit_withElement(&mut self, ctx: &WithElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#clusteringOrder}.
	 * @param ctx the parse tree
	 */
	fn visit_clusteringOrder(&mut self, ctx: &ClusteringOrderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#tableOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_tableOptions(&mut self, ctx: &TableOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#tableOptionItem}.
	 * @param ctx the parse tree
	 */
	fn visit_tableOptionItem(&mut self, ctx: &TableOptionItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#tableOptionName}.
	 * @param ctx the parse tree
	 */
	fn visit_tableOptionName(&mut self, ctx: &TableOptionNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#tableOptionValue}.
	 * @param ctx the parse tree
	 */
	fn visit_tableOptionValue(&mut self, ctx: &TableOptionValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#optionHash}.
	 * @param ctx the parse tree
	 */
	fn visit_optionHash(&mut self, ctx: &OptionHashContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#optionHashItem}.
	 * @param ctx the parse tree
	 */
	fn visit_optionHashItem(&mut self, ctx: &OptionHashItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#optionHashKey}.
	 * @param ctx the parse tree
	 */
	fn visit_optionHashKey(&mut self, ctx: &OptionHashKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#optionHashValue}.
	 * @param ctx the parse tree
	 */
	fn visit_optionHashValue(&mut self, ctx: &OptionHashValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#columnDefinitionList}.
	 * @param ctx the parse tree
	 */
	fn visit_columnDefinitionList(&mut self, ctx: &ColumnDefinitionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#columnDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_columnDefinition(&mut self, ctx: &ColumnDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#primaryKeyColumn}.
	 * @param ctx the parse tree
	 */
	fn visit_primaryKeyColumn(&mut self, ctx: &PrimaryKeyColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#primaryKeyElement}.
	 * @param ctx the parse tree
	 */
	fn visit_primaryKeyElement(&mut self, ctx: &PrimaryKeyElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#primaryKeyDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_primaryKeyDefinition(&mut self, ctx: &PrimaryKeyDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#singlePrimaryKey}.
	 * @param ctx the parse tree
	 */
	fn visit_singlePrimaryKey(&mut self, ctx: &SinglePrimaryKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#compoundKey}.
	 * @param ctx the parse tree
	 */
	fn visit_compoundKey(&mut self, ctx: &CompoundKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#compositeKey}.
	 * @param ctx the parse tree
	 */
	fn visit_compositeKey(&mut self, ctx: &CompositeKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#partitionKeyList}.
	 * @param ctx the parse tree
	 */
	fn visit_partitionKeyList(&mut self, ctx: &PartitionKeyListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#clusteringKeyList}.
	 * @param ctx the parse tree
	 */
	fn visit_clusteringKeyList(&mut self, ctx: &ClusteringKeyListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#partitionKey}.
	 * @param ctx the parse tree
	 */
	fn visit_partitionKey(&mut self, ctx: &PartitionKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#clusteringKey}.
	 * @param ctx the parse tree
	 */
	fn visit_clusteringKey(&mut self, ctx: &ClusteringKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#applyBatch}.
	 * @param ctx the parse tree
	 */
	fn visit_applyBatch(&mut self, ctx: &ApplyBatchContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#beginBatch}.
	 * @param ctx the parse tree
	 */
	fn visit_beginBatch(&mut self, ctx: &BeginBatchContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#batchType}.
	 * @param ctx the parse tree
	 */
	fn visit_batchType(&mut self, ctx: &BatchTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#alterKeyspace}.
	 * @param ctx the parse tree
	 */
	fn visit_alterKeyspace(&mut self, ctx: &AlterKeyspaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#replicationList}.
	 * @param ctx the parse tree
	 */
	fn visit_replicationList(&mut self, ctx: &ReplicationListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#replicationListItem}.
	 * @param ctx the parse tree
	 */
	fn visit_replicationListItem(&mut self, ctx: &ReplicationListItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#durableWrites}.
	 * @param ctx the parse tree
	 */
	fn visit_durableWrites(&mut self, ctx: &DurableWritesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#use_}.
	 * @param ctx the parse tree
	 */
	fn visit_use_(&mut self, ctx: &Use_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#truncate}.
	 * @param ctx the parse tree
	 */
	fn visit_truncate(&mut self, ctx: &TruncateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#createIndex}.
	 * @param ctx the parse tree
	 */
	fn visit_createIndex(&mut self, ctx: &CreateIndexContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#indexName}.
	 * @param ctx the parse tree
	 */
	fn visit_indexName(&mut self, ctx: &IndexNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#indexColumnSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_indexColumnSpec(&mut self, ctx: &IndexColumnSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#indexKeysSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_indexKeysSpec(&mut self, ctx: &IndexKeysSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#indexEntriesSSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_indexEntriesSSpec(&mut self, ctx: &IndexEntriesSSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#indexFullSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_indexFullSpec(&mut self, ctx: &IndexFullSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#delete_}.
	 * @param ctx the parse tree
	 */
	fn visit_delete_(&mut self, ctx: &Delete_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#deleteColumnList}.
	 * @param ctx the parse tree
	 */
	fn visit_deleteColumnList(&mut self, ctx: &DeleteColumnListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#deleteColumnItem}.
	 * @param ctx the parse tree
	 */
	fn visit_deleteColumnItem(&mut self, ctx: &DeleteColumnItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#update}.
	 * @param ctx the parse tree
	 */
	fn visit_update(&mut self, ctx: &UpdateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#ifSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_ifSpec(&mut self, ctx: &IfSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#ifConditionList}.
	 * @param ctx the parse tree
	 */
	fn visit_ifConditionList(&mut self, ctx: &IfConditionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#ifCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_ifCondition(&mut self, ctx: &IfConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#assignments}.
	 * @param ctx the parse tree
	 */
	fn visit_assignments(&mut self, ctx: &AssignmentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#assignmentElement}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentElement(&mut self, ctx: &AssignmentElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#assignmentSet}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentSet(&mut self, ctx: &AssignmentSetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#assignmentMap}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentMap(&mut self, ctx: &AssignmentMapContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#assignmentList}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentList(&mut self, ctx: &AssignmentListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#assignmentTuple}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentTuple(&mut self, ctx: &AssignmentTupleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#insert}.
	 * @param ctx the parse tree
	 */
	fn visit_insert(&mut self, ctx: &InsertContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#usingTtlTimestamp}.
	 * @param ctx the parse tree
	 */
	fn visit_usingTtlTimestamp(&mut self, ctx: &UsingTtlTimestampContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#timestamp}.
	 * @param ctx the parse tree
	 */
	fn visit_timestamp(&mut self, ctx: &TimestampContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#ttl}.
	 * @param ctx the parse tree
	 */
	fn visit_ttl(&mut self, ctx: &TtlContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#usingTimestampSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_usingTimestampSpec(&mut self, ctx: &UsingTimestampSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#ifNotExist}.
	 * @param ctx the parse tree
	 */
	fn visit_ifNotExist(&mut self, ctx: &IfNotExistContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#ifExist}.
	 * @param ctx the parse tree
	 */
	fn visit_ifExist(&mut self, ctx: &IfExistContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#insertValuesSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_insertValuesSpec(&mut self, ctx: &InsertValuesSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#insertColumnSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_insertColumnSpec(&mut self, ctx: &InsertColumnSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#columnList}.
	 * @param ctx the parse tree
	 */
	fn visit_columnList(&mut self, ctx: &ColumnListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#expressionList}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#select_}.
	 * @param ctx the parse tree
	 */
	fn visit_select_(&mut self, ctx: &Select_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#allowFilteringSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_allowFilteringSpec(&mut self, ctx: &AllowFilteringSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#limitSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_limitSpec(&mut self, ctx: &LimitSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#fromSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_fromSpec(&mut self, ctx: &FromSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#fromSpecElement}.
	 * @param ctx the parse tree
	 */
	fn visit_fromSpecElement(&mut self, ctx: &FromSpecElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#orderSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_orderSpec(&mut self, ctx: &OrderSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#orderSpecElement}.
	 * @param ctx the parse tree
	 */
	fn visit_orderSpecElement(&mut self, ctx: &OrderSpecElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#whereSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_whereSpec(&mut self, ctx: &WhereSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#distinctSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_distinctSpec(&mut self, ctx: &DistinctSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#selectElements}.
	 * @param ctx the parse tree
	 */
	fn visit_selectElements(&mut self, ctx: &SelectElementsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#selectElement}.
	 * @param ctx the parse tree
	 */
	fn visit_selectElement(&mut self, ctx: &SelectElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#relationElements}.
	 * @param ctx the parse tree
	 */
	fn visit_relationElements(&mut self, ctx: &RelationElementsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#relationElement}.
	 * @param ctx the parse tree
	 */
	fn visit_relationElement(&mut self, ctx: &RelationElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#relalationContains}.
	 * @param ctx the parse tree
	 */
	fn visit_relalationContains(&mut self, ctx: &RelalationContainsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#relalationContainsKey}.
	 * @param ctx the parse tree
	 */
	fn visit_relalationContainsKey(&mut self, ctx: &RelalationContainsKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#functionCall}.
	 * @param ctx the parse tree
	 */
	fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#functionArgs}.
	 * @param ctx the parse tree
	 */
	fn visit_functionArgs(&mut self, ctx: &FunctionArgsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#constant}.
	 * @param ctx the parse tree
	 */
	fn visit_constant(&mut self, ctx: &ConstantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#decimalLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_decimalLiteral(&mut self, ctx: &DecimalLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#floatLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_floatLiteral(&mut self, ctx: &FloatLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#stringLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#booleanLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#hexadecimalLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_hexadecimalLiteral(&mut self, ctx: &HexadecimalLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#keyspace}.
	 * @param ctx the parse tree
	 */
	fn visit_keyspace(&mut self, ctx: &KeyspaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#table}.
	 * @param ctx the parse tree
	 */
	fn visit_table(&mut self, ctx: &TableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#column}.
	 * @param ctx the parse tree
	 */
	fn visit_column(&mut self, ctx: &ColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#dataType}.
	 * @param ctx the parse tree
	 */
	fn visit_dataType(&mut self, ctx: &DataTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#dataTypeName}.
	 * @param ctx the parse tree
	 */
	fn visit_dataTypeName(&mut self, ctx: &DataTypeNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#dataTypeDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_dataTypeDefinition(&mut self, ctx: &DataTypeDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#orderDirection}.
	 * @param ctx the parse tree
	 */
	fn visit_orderDirection(&mut self, ctx: &OrderDirectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#role}.
	 * @param ctx the parse tree
	 */
	fn visit_role(&mut self, ctx: &RoleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#trigger}.
	 * @param ctx the parse tree
	 */
	fn visit_trigger(&mut self, ctx: &TriggerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#triggerClass}.
	 * @param ctx the parse tree
	 */
	fn visit_triggerClass(&mut self, ctx: &TriggerClassContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#materializedView}.
	 * @param ctx the parse tree
	 */
	fn visit_materializedView(&mut self, ctx: &MaterializedViewContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#type_}.
	 * @param ctx the parse tree
	 */
	fn visit_type_(&mut self, ctx: &Type_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#aggregate}.
	 * @param ctx the parse tree
	 */
	fn visit_aggregate(&mut self, ctx: &AggregateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#function_}.
	 * @param ctx the parse tree
	 */
	fn visit_function_(&mut self, ctx: &Function_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#language}.
	 * @param ctx the parse tree
	 */
	fn visit_language(&mut self, ctx: &LanguageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#user}.
	 * @param ctx the parse tree
	 */
	fn visit_user(&mut self, ctx: &UserContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#password}.
	 * @param ctx the parse tree
	 */
	fn visit_password(&mut self, ctx: &PasswordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#hashKey}.
	 * @param ctx the parse tree
	 */
	fn visit_hashKey(&mut self, ctx: &HashKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#param}.
	 * @param ctx the parse tree
	 */
	fn visit_param(&mut self, ctx: &ParamContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#paramName}.
	 * @param ctx the parse tree
	 */
	fn visit_paramName(&mut self, ctx: &ParamNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwAdd}.
	 * @param ctx the parse tree
	 */
	fn visit_kwAdd(&mut self, ctx: &KwAddContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwAggregate}.
	 * @param ctx the parse tree
	 */
	fn visit_kwAggregate(&mut self, ctx: &KwAggregateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwAll}.
	 * @param ctx the parse tree
	 */
	fn visit_kwAll(&mut self, ctx: &KwAllContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwAllPermissions}.
	 * @param ctx the parse tree
	 */
	fn visit_kwAllPermissions(&mut self, ctx: &KwAllPermissionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwAllow}.
	 * @param ctx the parse tree
	 */
	fn visit_kwAllow(&mut self, ctx: &KwAllowContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwAlter}.
	 * @param ctx the parse tree
	 */
	fn visit_kwAlter(&mut self, ctx: &KwAlterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwAnd}.
	 * @param ctx the parse tree
	 */
	fn visit_kwAnd(&mut self, ctx: &KwAndContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwApply}.
	 * @param ctx the parse tree
	 */
	fn visit_kwApply(&mut self, ctx: &KwApplyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwAs}.
	 * @param ctx the parse tree
	 */
	fn visit_kwAs(&mut self, ctx: &KwAsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwAsc}.
	 * @param ctx the parse tree
	 */
	fn visit_kwAsc(&mut self, ctx: &KwAscContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwAuthorize}.
	 * @param ctx the parse tree
	 */
	fn visit_kwAuthorize(&mut self, ctx: &KwAuthorizeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwBatch}.
	 * @param ctx the parse tree
	 */
	fn visit_kwBatch(&mut self, ctx: &KwBatchContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwBegin}.
	 * @param ctx the parse tree
	 */
	fn visit_kwBegin(&mut self, ctx: &KwBeginContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwBy}.
	 * @param ctx the parse tree
	 */
	fn visit_kwBy(&mut self, ctx: &KwByContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwCalled}.
	 * @param ctx the parse tree
	 */
	fn visit_kwCalled(&mut self, ctx: &KwCalledContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwClustering}.
	 * @param ctx the parse tree
	 */
	fn visit_kwClustering(&mut self, ctx: &KwClusteringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwCompact}.
	 * @param ctx the parse tree
	 */
	fn visit_kwCompact(&mut self, ctx: &KwCompactContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwContains}.
	 * @param ctx the parse tree
	 */
	fn visit_kwContains(&mut self, ctx: &KwContainsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwCreate}.
	 * @param ctx the parse tree
	 */
	fn visit_kwCreate(&mut self, ctx: &KwCreateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwDelete}.
	 * @param ctx the parse tree
	 */
	fn visit_kwDelete(&mut self, ctx: &KwDeleteContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwDesc}.
	 * @param ctx the parse tree
	 */
	fn visit_kwDesc(&mut self, ctx: &KwDescContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwDescibe}.
	 * @param ctx the parse tree
	 */
	fn visit_kwDescibe(&mut self, ctx: &KwDescibeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwDistinct}.
	 * @param ctx the parse tree
	 */
	fn visit_kwDistinct(&mut self, ctx: &KwDistinctContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwDrop}.
	 * @param ctx the parse tree
	 */
	fn visit_kwDrop(&mut self, ctx: &KwDropContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwDurableWrites}.
	 * @param ctx the parse tree
	 */
	fn visit_kwDurableWrites(&mut self, ctx: &KwDurableWritesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwEntries}.
	 * @param ctx the parse tree
	 */
	fn visit_kwEntries(&mut self, ctx: &KwEntriesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwExecute}.
	 * @param ctx the parse tree
	 */
	fn visit_kwExecute(&mut self, ctx: &KwExecuteContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwExists}.
	 * @param ctx the parse tree
	 */
	fn visit_kwExists(&mut self, ctx: &KwExistsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwFiltering}.
	 * @param ctx the parse tree
	 */
	fn visit_kwFiltering(&mut self, ctx: &KwFilteringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwFinalfunc}.
	 * @param ctx the parse tree
	 */
	fn visit_kwFinalfunc(&mut self, ctx: &KwFinalfuncContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwFrom}.
	 * @param ctx the parse tree
	 */
	fn visit_kwFrom(&mut self, ctx: &KwFromContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwFull}.
	 * @param ctx the parse tree
	 */
	fn visit_kwFull(&mut self, ctx: &KwFullContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwFunction}.
	 * @param ctx the parse tree
	 */
	fn visit_kwFunction(&mut self, ctx: &KwFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwFunctions}.
	 * @param ctx the parse tree
	 */
	fn visit_kwFunctions(&mut self, ctx: &KwFunctionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwGrant}.
	 * @param ctx the parse tree
	 */
	fn visit_kwGrant(&mut self, ctx: &KwGrantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwIf}.
	 * @param ctx the parse tree
	 */
	fn visit_kwIf(&mut self, ctx: &KwIfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwIn}.
	 * @param ctx the parse tree
	 */
	fn visit_kwIn(&mut self, ctx: &KwInContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwIndex}.
	 * @param ctx the parse tree
	 */
	fn visit_kwIndex(&mut self, ctx: &KwIndexContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwInitcond}.
	 * @param ctx the parse tree
	 */
	fn visit_kwInitcond(&mut self, ctx: &KwInitcondContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwInput}.
	 * @param ctx the parse tree
	 */
	fn visit_kwInput(&mut self, ctx: &KwInputContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwInsert}.
	 * @param ctx the parse tree
	 */
	fn visit_kwInsert(&mut self, ctx: &KwInsertContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwInto}.
	 * @param ctx the parse tree
	 */
	fn visit_kwInto(&mut self, ctx: &KwIntoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwIs}.
	 * @param ctx the parse tree
	 */
	fn visit_kwIs(&mut self, ctx: &KwIsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwJson}.
	 * @param ctx the parse tree
	 */
	fn visit_kwJson(&mut self, ctx: &KwJsonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwKey}.
	 * @param ctx the parse tree
	 */
	fn visit_kwKey(&mut self, ctx: &KwKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwKeys}.
	 * @param ctx the parse tree
	 */
	fn visit_kwKeys(&mut self, ctx: &KwKeysContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwKeyspace}.
	 * @param ctx the parse tree
	 */
	fn visit_kwKeyspace(&mut self, ctx: &KwKeyspaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwKeyspaces}.
	 * @param ctx the parse tree
	 */
	fn visit_kwKeyspaces(&mut self, ctx: &KwKeyspacesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwLanguage}.
	 * @param ctx the parse tree
	 */
	fn visit_kwLanguage(&mut self, ctx: &KwLanguageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwLimit}.
	 * @param ctx the parse tree
	 */
	fn visit_kwLimit(&mut self, ctx: &KwLimitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwList}.
	 * @param ctx the parse tree
	 */
	fn visit_kwList(&mut self, ctx: &KwListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwLogged}.
	 * @param ctx the parse tree
	 */
	fn visit_kwLogged(&mut self, ctx: &KwLoggedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwLogin}.
	 * @param ctx the parse tree
	 */
	fn visit_kwLogin(&mut self, ctx: &KwLoginContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwMaterialized}.
	 * @param ctx the parse tree
	 */
	fn visit_kwMaterialized(&mut self, ctx: &KwMaterializedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwModify}.
	 * @param ctx the parse tree
	 */
	fn visit_kwModify(&mut self, ctx: &KwModifyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwNosuperuser}.
	 * @param ctx the parse tree
	 */
	fn visit_kwNosuperuser(&mut self, ctx: &KwNosuperuserContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwNorecursive}.
	 * @param ctx the parse tree
	 */
	fn visit_kwNorecursive(&mut self, ctx: &KwNorecursiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwNot}.
	 * @param ctx the parse tree
	 */
	fn visit_kwNot(&mut self, ctx: &KwNotContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwNull}.
	 * @param ctx the parse tree
	 */
	fn visit_kwNull(&mut self, ctx: &KwNullContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwOf}.
	 * @param ctx the parse tree
	 */
	fn visit_kwOf(&mut self, ctx: &KwOfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwOn}.
	 * @param ctx the parse tree
	 */
	fn visit_kwOn(&mut self, ctx: &KwOnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_kwOptions(&mut self, ctx: &KwOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwOr}.
	 * @param ctx the parse tree
	 */
	fn visit_kwOr(&mut self, ctx: &KwOrContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwOrder}.
	 * @param ctx the parse tree
	 */
	fn visit_kwOrder(&mut self, ctx: &KwOrderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwPassword}.
	 * @param ctx the parse tree
	 */
	fn visit_kwPassword(&mut self, ctx: &KwPasswordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwPrimary}.
	 * @param ctx the parse tree
	 */
	fn visit_kwPrimary(&mut self, ctx: &KwPrimaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwRename}.
	 * @param ctx the parse tree
	 */
	fn visit_kwRename(&mut self, ctx: &KwRenameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwReplace}.
	 * @param ctx the parse tree
	 */
	fn visit_kwReplace(&mut self, ctx: &KwReplaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwReplication}.
	 * @param ctx the parse tree
	 */
	fn visit_kwReplication(&mut self, ctx: &KwReplicationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwReturns}.
	 * @param ctx the parse tree
	 */
	fn visit_kwReturns(&mut self, ctx: &KwReturnsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwRole}.
	 * @param ctx the parse tree
	 */
	fn visit_kwRole(&mut self, ctx: &KwRoleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwRoles}.
	 * @param ctx the parse tree
	 */
	fn visit_kwRoles(&mut self, ctx: &KwRolesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwSelect}.
	 * @param ctx the parse tree
	 */
	fn visit_kwSelect(&mut self, ctx: &KwSelectContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwSet}.
	 * @param ctx the parse tree
	 */
	fn visit_kwSet(&mut self, ctx: &KwSetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwSfunc}.
	 * @param ctx the parse tree
	 */
	fn visit_kwSfunc(&mut self, ctx: &KwSfuncContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwStorage}.
	 * @param ctx the parse tree
	 */
	fn visit_kwStorage(&mut self, ctx: &KwStorageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwStype}.
	 * @param ctx the parse tree
	 */
	fn visit_kwStype(&mut self, ctx: &KwStypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwSuperuser}.
	 * @param ctx the parse tree
	 */
	fn visit_kwSuperuser(&mut self, ctx: &KwSuperuserContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwTable}.
	 * @param ctx the parse tree
	 */
	fn visit_kwTable(&mut self, ctx: &KwTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwTimestamp}.
	 * @param ctx the parse tree
	 */
	fn visit_kwTimestamp(&mut self, ctx: &KwTimestampContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwTo}.
	 * @param ctx the parse tree
	 */
	fn visit_kwTo(&mut self, ctx: &KwToContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwTrigger}.
	 * @param ctx the parse tree
	 */
	fn visit_kwTrigger(&mut self, ctx: &KwTriggerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwTruncate}.
	 * @param ctx the parse tree
	 */
	fn visit_kwTruncate(&mut self, ctx: &KwTruncateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwTtl}.
	 * @param ctx the parse tree
	 */
	fn visit_kwTtl(&mut self, ctx: &KwTtlContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwType}.
	 * @param ctx the parse tree
	 */
	fn visit_kwType(&mut self, ctx: &KwTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwUnlogged}.
	 * @param ctx the parse tree
	 */
	fn visit_kwUnlogged(&mut self, ctx: &KwUnloggedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwUpdate}.
	 * @param ctx the parse tree
	 */
	fn visit_kwUpdate(&mut self, ctx: &KwUpdateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwUse}.
	 * @param ctx the parse tree
	 */
	fn visit_kwUse(&mut self, ctx: &KwUseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwUser}.
	 * @param ctx the parse tree
	 */
	fn visit_kwUser(&mut self, ctx: &KwUserContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwUsers}.
	 * @param ctx the parse tree
	 */
	fn visit_kwUsers(&mut self, ctx: &KwUsersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwUsing}.
	 * @param ctx the parse tree
	 */
	fn visit_kwUsing(&mut self, ctx: &KwUsingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwValues}.
	 * @param ctx the parse tree
	 */
	fn visit_kwValues(&mut self, ctx: &KwValuesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwView}.
	 * @param ctx the parse tree
	 */
	fn visit_kwView(&mut self, ctx: &KwViewContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwWhere}.
	 * @param ctx the parse tree
	 */
	fn visit_kwWhere(&mut self, ctx: &KwWhereContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwWith}.
	 * @param ctx the parse tree
	 */
	fn visit_kwWith(&mut self, ctx: &KwWithContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#kwRevoke}.
	 * @param ctx the parse tree
	 */
	fn visit_kwRevoke(&mut self, ctx: &KwRevokeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#eof}.
	 * @param ctx the parse tree
	 */
	fn visit_eof(&mut self, ctx: &EofContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#syntaxBracketLr}.
	 * @param ctx the parse tree
	 */
	fn visit_syntaxBracketLr(&mut self, ctx: &SyntaxBracketLrContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#syntaxBracketRr}.
	 * @param ctx the parse tree
	 */
	fn visit_syntaxBracketRr(&mut self, ctx: &SyntaxBracketRrContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#syntaxBracketLc}.
	 * @param ctx the parse tree
	 */
	fn visit_syntaxBracketLc(&mut self, ctx: &SyntaxBracketLcContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#syntaxBracketRc}.
	 * @param ctx the parse tree
	 */
	fn visit_syntaxBracketRc(&mut self, ctx: &SyntaxBracketRcContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#syntaxBracketLa}.
	 * @param ctx the parse tree
	 */
	fn visit_syntaxBracketLa(&mut self, ctx: &SyntaxBracketLaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#syntaxBracketRa}.
	 * @param ctx the parse tree
	 */
	fn visit_syntaxBracketRa(&mut self, ctx: &SyntaxBracketRaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#syntaxBracketLs}.
	 * @param ctx the parse tree
	 */
	fn visit_syntaxBracketLs(&mut self, ctx: &SyntaxBracketLsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#syntaxBracketRs}.
	 * @param ctx the parse tree
	 */
	fn visit_syntaxBracketRs(&mut self, ctx: &SyntaxBracketRsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#syntaxComma}.
	 * @param ctx the parse tree
	 */
	fn visit_syntaxComma(&mut self, ctx: &SyntaxCommaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CqlParser#syntaxColon}.
	 * @param ctx the parse tree
	 */
	fn visit_syntaxColon(&mut self, ctx: &SyntaxColonContext<'input>) { self.visit_children(ctx) }


}