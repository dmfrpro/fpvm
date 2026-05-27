pub(crate) struct Brancher {
    index: i64,
    branch_type: BranchType,
}

pub(crate) struct BrancherStack {
    pub(crate) index: i64,
    pub(crate) loop_branches: Vec<String>,
}

pub(crate) enum BranchType {
    Cond,
    While,
    Prog,
}

impl ToString for BranchType {
    fn to_string(&self) -> String {
        match self {
            BranchType::Cond => "cond",
            BranchType::While => "while",
            BranchType::Prog => "prog",
        }
        .to_string()
    }
}

impl Brancher {
    pub(crate) fn get_label(&self, label_suffix: String) -> String {
        format!(
            "{}_{}_{}",
            self.branch_type.to_string(),
            self.index,
            label_suffix
        )
    }
}

impl BrancherStack {
    pub(crate) fn push_brancher(&mut self, brancher: String) {
        self.loop_branches.push(brancher);
    }

    pub(crate) fn pop_brancher(&mut self) {
        self.loop_branches.pop();
    }

    pub(crate) fn peek_brancher(&self) -> Option<&String> {
        self.loop_branches.last()
    }

    pub(crate) fn new_brancher(&mut self, branch_type: BranchType) -> Brancher {
        self.index += 1;
        Brancher {
            index: self.index,
            branch_type: branch_type,
        }
    }
}
