use {
    super::{NodeData, Prebuild},
    crate::{
        ast_builder::{
            ExprNode, FilterNode, GroupByNode, HashJoinNode, HavingNode, JoinConstraintNode,
            JoinNode, LimitOffsetNode, OrderByNode, ProjectNode, SelectItemList, SelectNode,
        },
        result::Result,
    },
};

#[derive(Clone)]
pub enum PrevNode {
    Select(SelectNode),
    GroupBy(GroupByNode),
    Having(HavingNode),
    Join(Box<JoinNode>),
    JoinConstraint(Box<JoinConstraintNode>),
    HashJoin(HashJoinNode),
    Filter(FilterNode),
    OrderBy(OrderByNode),
}

impl Prebuild for PrevNode {
    fn prebuild(self) -> Result<NodeData> {
        match self {
            Self::Select(node) => node.prebuild(),
            Self::GroupBy(node) => node.prebuild(),
            Self::Having(node) => node.prebuild(),
            Self::Join(node) => node.prebuild(),
            Self::JoinConstraint(node) => node.prebuild(),
            Self::HashJoin(node) => node.prebuild(),
            Self::Filter(node) => node.prebuild(),
            Self::OrderBy(node) => node.prebuild(),
        }
    }
}

impl From<SelectNode> for PrevNode {
    fn from(node: SelectNode) -> Self {
        PrevNode::Select(node)
    }
}

impl From<GroupByNode> for PrevNode {
    fn from(node: GroupByNode) -> Self {
        PrevNode::GroupBy(node)
    }
}

impl From<HavingNode> for PrevNode {
    fn from(node: HavingNode) -> Self {
        PrevNode::Having(node)
    }
}

impl From<JoinConstraintNode> for PrevNode {
    fn from(node: JoinConstraintNode) -> Self {
        PrevNode::JoinConstraint(Box::new(node))
    }
}

impl From<JoinNode> for PrevNode {
    fn from(node: JoinNode) -> Self {
        PrevNode::Join(Box::new(node))
    }
}

impl From<HashJoinNode> for PrevNode {
    fn from(node: HashJoinNode) -> Self {
        PrevNode::HashJoin(node)
    }
}

impl From<FilterNode> for PrevNode {
    fn from(node: FilterNode) -> Self {
        PrevNode::Filter(node)
    }
}

impl From<OrderByNode> for PrevNode {
    fn from(node: OrderByNode) -> Self {
        PrevNode::OrderBy(node)
    }
}

#[derive(Clone)]
pub struct LimitNode {
    prev_node: PrevNode,
    expr: ExprNode,
}

impl LimitNode {
    pub fn new<N: Into<PrevNode>, T: Into<ExprNode>>(prev_node: N, expr: T) -> Self {
        Self {
            prev_node: prev_node.into(),
            expr: expr.into(),
        }
    }

    pub fn offset<T: Into<ExprNode>>(self, expr: T) -> LimitOffsetNode {
        LimitOffsetNode::new(self, expr)
    }

    pub fn project<T: Into<SelectItemList>>(self, select_items: T) -> ProjectNode {
        ProjectNode::new(self, select_items)
    }
}

impl Prebuild for LimitNode {
    fn prebuild(self) -> Result<NodeData> {
        let mut select_data = self.prev_node.prebuild()?;
        select_data.limit = Some(self.expr.try_into()?);

        Ok(select_data)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            Join, JoinConstraint, JoinExecutor, JoinOperator, Query, Select, SetExpr, Statement,
            TableFactor, TableWithJoins,
        },
        ast_builder::{col, num, table, test, Build, SelectItemList},
    };

    #[test]
    fn limit() {
        // select node -> limit node -> build
        let actual = table("Foo").select().limit(10).build();
        let expected = "SELECT * FROM Foo LIMIT 10";
        test(actual, expected);

        // group by node -> limit node -> build
        let actual = table("Foo").select().group_by("bar").limit(10).build();
        let expected = "SELECT * FROM Foo GROUP BY bar LIMIT 10";
        test(actual, expected);

        // having node -> limit node -> build
        let actual = table("Foo")
            .select()
            .group_by("bar")
            .having("bar = 10")
            .limit(10)
            .build();
        let expected = "SELECT * FROM Foo GROUP BY bar HAVING bar = 10 LIMIT 10";
        test(actual, expected);

        // join node -> limit node -> build
        let actual = table("Foo").select().join("Bar").limit(10).build();
        let expected = "SELECT * FROM Foo JOIN Bar LIMIT 10";
        test(actual, expected);

        // join node -> limit node -> build
        let actual = table("Foo").select().join_as("Bar", "B").limit(10).build();
        let expected = "SELECT * FROM Foo JOIN Bar AS B LIMIT 10";
        test(actual, expected);

        // join node -> limit node -> build
        let actual = table("Foo").select().left_join("Bar").limit(10).build();
        let expected = "SELECT * FROM Foo LEFT JOIN Bar LIMIT 10";
        test(actual, expected);

        // join node -> limit node -> build
        let actual = table("Foo")
            .select()
            .left_join_as("Bar", "B")
            .limit(10)
            .build();
        let expected = "SELECT * FROM Foo LEFT JOIN Bar AS B LIMIT 10";
        test(actual, expected);

        // group by node -> limit node -> build
        let actual = table("Foo").select().group_by("id").limit(10).build();
        let expected = "SELECT * FROM Foo GROUP BY id LIMIT 10";
        test(actual, expected);

        // having node -> limit node -> build
        let actual = table("Foo")
            .select()
            .group_by("id")
            .having(col("id").gt(10))
            .limit(10)
            .build();
        let expected = "SELECT * FROM Foo GROUP BY id HAVING id > 10 LIMIT 10";
        test(actual, expected);

        // join constraint node -> limit node -> build
        let actual = table("Foo")
            .select()
            .join("Bar")
            .on("Foo.id = Bar.id")
            .limit(10)
            .build();
        let expected = "SELECT * FROM Foo JOIN Bar ON Foo.id = Bar.id LIMIT 10";
        test(actual, expected);

        // filter node -> limit node -> build
        let actual = table("World")
            .select()
            .filter(col("id").gt(2))
            .limit(100)
            .build();
        let expected = "SELECT * FROM World WHERE id > 2 LIMIT 100";
        test(actual, expected);

        // hash join node -> limit node -> build
        let actual = table("Player")
            .select()
            .join("PlayerItem")
            .hash_executor("PlayerItem.user_id", "Player.id")
            .limit(100)
            .build();
        let expected = {
            let join = Join {
                relation: TableFactor::Table {
                    name: "PlayerItem".to_owned(),
                    alias: None,
                    index: None,
                },
                join_operator: JoinOperator::Inner(JoinConstraint::None),
                join_executor: JoinExecutor::Hash {
                    key_expr: col("PlayerItem.user_id").try_into().unwrap(),
                    value_expr: col("Player.id").try_into().unwrap(),
                    where_clause: None,
                },
            };
            let select = Select {
                projection: SelectItemList::from("*").try_into().unwrap(),
                from: TableWithJoins {
                    relation: TableFactor::Table {
                        name: "Player".to_owned(),
                        alias: None,
                        index: None,
                    },
                    joins: vec![join],
                },
                selection: None,
                group_by: Vec::new(),
                having: None,
            };

            Ok(Statement::Query(Query {
                body: SetExpr::Select(Box::new(select)),
                order_by: Vec::new(),
                limit: Some(num(100).try_into().unwrap()),
                offset: None,
            }))
        };
        assert_eq!(actual, expected);
    }
}
