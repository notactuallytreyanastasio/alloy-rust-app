#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            Ok(())
    }).clone()
}
struct ChangesetErrorStruct {
    field: std::sync::Arc<String>, message: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct ChangesetError(std::sync::Arc<ChangesetErrorStruct>);
#[derive(Clone)]
pub struct ChangesetErrorBuilder {
    pub field: std::sync::Arc<String>, pub message: std::sync::Arc<String>
}
impl ChangesetErrorBuilder {
    pub fn build(self) -> ChangesetError {
        ChangesetError::new(self.field, self.message)
    }
}
impl ChangesetError {
    pub fn new(field__717: impl temper_core::ToArcString, message__718: impl temper_core::ToArcString) -> ChangesetError {
        let field__717 = field__717.to_arc_string();
        let message__718 = message__718.to_arc_string();
        let field;
        let message;
        field = field__717.clone();
        message = message__718.clone();
        let selfish = ChangesetError(std::sync::Arc::new(ChangesetErrorStruct {
                    field, message
        }));
        return selfish;
    }
    pub fn field(& self) -> std::sync::Arc<String> {
        return self.0.field.clone();
    }
    pub fn message(& self) -> std::sync::Arc<String> {
        return self.0.message.clone();
    }
}
temper_core::impl_any_value_trait!(ChangesetError, []);
struct NumberValidationOptsStruct {
    greater_than: Option<f64>, less_than: Option<f64>, greater_than_or_equal: Option<f64>, less_than_or_equal: Option<f64>, equal_to: Option<f64>
}
#[derive(Clone)]
pub struct NumberValidationOpts(std::sync::Arc<NumberValidationOptsStruct>);
#[derive(Clone)]
pub struct NumberValidationOptsBuilder {
    pub greater_than: Option<f64>, pub less_than: Option<f64>, pub greater_than_or_equal: Option<f64>, pub less_than_or_equal: Option<f64>, pub equal_to: Option<f64>
}
impl NumberValidationOptsBuilder {
    pub fn build(self) -> NumberValidationOpts {
        NumberValidationOpts::new(self.greater_than, self.less_than, self.greater_than_or_equal, self.less_than_or_equal, self.equal_to)
    }
}
impl NumberValidationOpts {
    pub fn new(greaterThan__725: Option<f64>, lessThan__726: Option<f64>, greaterThanOrEqual__727: Option<f64>, lessThanOrEqual__728: Option<f64>, equalTo__729: Option<f64>) -> NumberValidationOpts {
        let greater_than;
        let less_than;
        let greater_than_or_equal;
        let less_than_or_equal;
        let equal_to;
        greater_than = greaterThan__725;
        less_than = lessThan__726;
        greater_than_or_equal = greaterThanOrEqual__727;
        less_than_or_equal = lessThanOrEqual__728;
        equal_to = equalTo__729;
        let selfish = NumberValidationOpts(std::sync::Arc::new(NumberValidationOptsStruct {
                    greater_than, less_than, greater_than_or_equal, less_than_or_equal, equal_to
        }));
        return selfish;
    }
    pub fn greater_than(& self) -> Option<f64> {
        return self.0.greater_than;
    }
    pub fn less_than(& self) -> Option<f64> {
        return self.0.less_than;
    }
    pub fn greater_than_or_equal(& self) -> Option<f64> {
        return self.0.greater_than_or_equal;
    }
    pub fn less_than_or_equal(& self) -> Option<f64> {
        return self.0.less_than_or_equal;
    }
    pub fn equal_to(& self) -> Option<f64> {
        return self.0.equal_to;
    }
}
temper_core::impl_any_value_trait!(NumberValidationOpts, []);
pub enum ChangesetEnum {
    ChangesetImpl(ChangesetImpl)
}
pub trait ChangesetTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> ChangesetEnum;
    fn clone_boxed(& self) -> Changeset;
    fn table_def(& self) -> TableDef;
    fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
    fn errors(& self) -> temper_core::List<ChangesetError>;
    fn is_valid(& self) -> bool;
    fn cast(& self, allowedFields__739: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_required(& self, fields__742: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_length(& self, field__745: SafeIdentifier, min__746: i32, max__747: i32) -> Changeset;
    fn validate_int(& self, field__750: SafeIdentifier) -> Changeset;
    fn validate_int64(& self, field__753: SafeIdentifier) -> Changeset;
    fn validate_float(& self, field__756: SafeIdentifier) -> Changeset;
    fn validate_bool(& self, field__759: SafeIdentifier) -> Changeset;
    fn put_change(& self, field__762: SafeIdentifier, value__763: std::sync::Arc<String>) -> Changeset;
    fn get_change(& self, field__766: SafeIdentifier) -> temper_core::Result<std::sync::Arc<String>>;
    fn delete_change(& self, field__769: SafeIdentifier) -> Changeset;
    fn validate_inclusion(& self, field__772: SafeIdentifier, allowed__773: temper_core::List<std::sync::Arc<String>>) -> Changeset;
    fn validate_exclusion(& self, field__776: SafeIdentifier, disallowed__777: temper_core::List<std::sync::Arc<String>>) -> Changeset;
    fn validate_number(& self, field__780: SafeIdentifier, opts__781: NumberValidationOpts) -> Changeset;
    fn validate_acceptance(& self, field__784: SafeIdentifier) -> Changeset;
    fn validate_confirmation(& self, field__787: SafeIdentifier, confirmationField__788: SafeIdentifier) -> Changeset;
    fn validate_contains(& self, field__791: SafeIdentifier, substring__792: std::sync::Arc<String>) -> Changeset;
    fn validate_starts_with(& self, field__795: SafeIdentifier, prefix__796: std::sync::Arc<String>) -> Changeset;
    fn validate_ends_with(& self, field__799: SafeIdentifier, suffix__800: std::sync::Arc<String>) -> Changeset;
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment>;
    fn to_update_sql(& self, id__805: i32) -> temper_core::Result<SqlFragment>;
}
#[derive(Clone)]
pub struct Changeset(std::sync::Arc<dyn ChangesetTrait>);
impl Changeset {
    pub fn new(selfish: impl ChangesetTrait + 'static) -> Changeset {
        Changeset(std::sync::Arc::new(selfish))
    }
}
impl ChangesetTrait for Changeset {
    fn as_enum(& self) -> ChangesetEnum {
        ChangesetTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> Changeset {
        ChangesetTrait::clone_boxed( & ( * self.0))
    }
    fn table_def(& self) -> TableDef {
        ChangesetTrait::table_def( & ( * self.0))
    }
    fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> {
        ChangesetTrait::changes( & ( * self.0))
    }
    fn errors(& self) -> temper_core::List<ChangesetError> {
        ChangesetTrait::errors( & ( * self.0))
    }
    fn is_valid(& self) -> bool {
        ChangesetTrait::is_valid( & ( * self.0))
    }
    fn cast(& self, arg1: temper_core::List<SafeIdentifier>) -> Changeset {
        ChangesetTrait::cast( & ( * self.0), arg1)
    }
    fn validate_required(& self, arg1: temper_core::List<SafeIdentifier>) -> Changeset {
        ChangesetTrait::validate_required( & ( * self.0), arg1)
    }
    fn validate_length(& self, arg1: SafeIdentifier, arg2: i32, arg3: i32) -> Changeset {
        ChangesetTrait::validate_length( & ( * self.0), arg1, arg2, arg3)
    }
    fn validate_int(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_int( & ( * self.0), arg1)
    }
    fn validate_int64(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_int64( & ( * self.0), arg1)
    }
    fn validate_float(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_float( & ( * self.0), arg1)
    }
    fn validate_bool(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_bool( & ( * self.0), arg1)
    }
    fn put_change(& self, arg1: SafeIdentifier, arg2: std::sync::Arc<String>) -> Changeset {
        ChangesetTrait::put_change( & ( * self.0), arg1, arg2)
    }
    fn get_change(& self, arg1: SafeIdentifier) -> temper_core::Result<std::sync::Arc<String>> {
        ChangesetTrait::get_change( & ( * self.0), arg1)
    }
    fn delete_change(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::delete_change( & ( * self.0), arg1)
    }
    fn validate_inclusion(& self, arg1: SafeIdentifier, arg2: temper_core::List<std::sync::Arc<String>>) -> Changeset {
        ChangesetTrait::validate_inclusion( & ( * self.0), arg1, arg2)
    }
    fn validate_exclusion(& self, arg1: SafeIdentifier, arg2: temper_core::List<std::sync::Arc<String>>) -> Changeset {
        ChangesetTrait::validate_exclusion( & ( * self.0), arg1, arg2)
    }
    fn validate_number(& self, arg1: SafeIdentifier, arg2: NumberValidationOpts) -> Changeset {
        ChangesetTrait::validate_number( & ( * self.0), arg1, arg2)
    }
    fn validate_acceptance(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_acceptance( & ( * self.0), arg1)
    }
    fn validate_confirmation(& self, arg1: SafeIdentifier, arg2: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_confirmation( & ( * self.0), arg1, arg2)
    }
    fn validate_contains(& self, arg1: SafeIdentifier, arg2: std::sync::Arc<String>) -> Changeset {
        ChangesetTrait::validate_contains( & ( * self.0), arg1, arg2)
    }
    fn validate_starts_with(& self, arg1: SafeIdentifier, arg2: std::sync::Arc<String>) -> Changeset {
        ChangesetTrait::validate_starts_with( & ( * self.0), arg1, arg2)
    }
    fn validate_ends_with(& self, arg1: SafeIdentifier, arg2: std::sync::Arc<String>) -> Changeset {
        ChangesetTrait::validate_ends_with( & ( * self.0), arg1, arg2)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        ChangesetTrait::to_insert_sql( & ( * self.0))
    }
    fn to_update_sql(& self, arg1: i32) -> temper_core::Result<SqlFragment> {
        ChangesetTrait::to_update_sql( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(Changeset);
impl std::ops::Deref for Changeset {
    type Target = dyn ChangesetTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct ChangesetImplStruct {
    table_def: TableDef, params: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, changes: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, errors: temper_core::List<ChangesetError>, is_valid: bool
}
#[derive(Clone)]
pub (crate) struct ChangesetImpl(std::sync::Arc<ChangesetImplStruct>);
impl ChangesetImpl {
    pub fn table_def(& self) -> TableDef {
        return self.0.table_def.clone();
    }
    pub fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> {
        return self.0.changes.clone();
    }
    pub fn errors(& self) -> temper_core::List<ChangesetError> {
        return self.0.errors.clone();
    }
    pub fn is_valid(& self) -> bool {
        return self.0.is_valid;
    }
    fn add_error(& self, field__821: impl temper_core::ToArcString, message__822: impl temper_core::ToArcString) -> Changeset {
        let field__821 = field__821.to_arc_string();
        let message__822 = message__822.to_arc_string();
        let eb__824: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
        temper_core::listed::add( & eb__824, ChangesetError::new(field__821.clone(), message__822.clone()), None);
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), self.0.changes.clone(), temper_core::ListedTrait::to_list( & eb__824), false));
    }
    pub fn cast(& self, allowedFields__826: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let allowedFields__826 = allowedFields__826.to_list();
        let mb__828: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        #[derive(Clone)]
        struct ClosureGroup___1 {
            this__292: ChangesetImpl, mb__828: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>>
        }
        impl ClosureGroup___1 {
            fn fn__17273(& self, f__829: SafeIdentifier) {
                let mut t___17271: std::sync::Arc<String>;
                let mut t___17268: std::sync::Arc<String> = f__829.sql_value();
                let val__830: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.this__292.0.params, t___17268.clone(), std::sync::Arc::new("".to_string()));
                if ! val__830.is_empty() {
                    t___17271 = f__829.sql_value();
                    temper_core::MapBuilder::set( & self.mb__828, t___17271.clone(), val__830.clone());
                }
            }
        }
        let closure_group = ClosureGroup___1 {
            this__292: self.clone(), mb__828: mb__828.clone()
        };
        let fn__17273 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__829: SafeIdentifier | closure_group.fn__17273(f__829))
        };
        temper_core::listed::list_for_each( & allowedFields__826, & ( * fn__17273.clone()));
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__828), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn validate_required(& self, fields__832: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let fields__832 = fields__832.to_list();
        let return__461: Changeset;
        let mut t___17266: temper_core::List<ChangesetError>;
        let mut t___9729: TableDef;
        let mut t___9730: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___9731: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__833: {
            if ! self.0.is_valid {
                return__461 = Changeset::new(self.clone());
                break 'fn__833;
            }
            let eb__834: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
            let mut valid__835: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___2 {
                this__293: ChangesetImpl, eb__834: temper_core::ListBuilder<ChangesetError>, valid__835: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___2 {
                fn fn__17262(& self, f__836: SafeIdentifier) {
                    let mut t___17260: ChangesetError;
                    let mut t___17257: std::sync::Arc<String> = f__836.sql_value();
                    if ! temper_core::MappedTrait::has( & self.this__293.0.changes, t___17257.clone()) {
                        t___17260 = ChangesetError::new(f__836.sql_value(), "is required");
                        temper_core::listed::add( & self.eb__834, t___17260.clone(), None);
                        {
                            * self.valid__835.write().unwrap() = false;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___2 {
                this__293: self.clone(), eb__834: eb__834.clone(), valid__835: valid__835.clone()
            };
            let fn__17262 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__836: SafeIdentifier | closure_group.fn__17262(f__836))
            };
            temper_core::listed::list_for_each( & fields__832, & ( * fn__17262.clone()));
            t___9729 = self.0.table_def.clone();
            t___9730 = self.0.params.clone();
            t___9731 = self.0.changes.clone();
            t___17266 = temper_core::ListedTrait::to_list( & eb__834);
            return__461 = Changeset::new(ChangesetImpl::new(t___9729.clone(), t___9730.clone(), t___9731.clone(), t___17266.clone(), temper_core::read_locked( & valid__835)));
        }
        return return__461.clone();
    }
    pub fn validate_length(& self, field__838: SafeIdentifier, min__839: i32, max__840: i32) -> Changeset {
        let return__462: Changeset;
        let mut t___17248: std::sync::Arc<String>;
        let mut t___17253: std::sync::Arc<String>;
        let mut t___17254: std::sync::Arc<String>;
        let mut t___17255: std::sync::Arc<String>;
        let mut t___9719: bool;
        'fn__841: {
            if ! self.0.is_valid {
                return__462 = Changeset::new(self.clone());
                break 'fn__841;
            }
            t___17248 = field__838.sql_value();
            let val__842: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___17248.clone(), std::sync::Arc::new("".to_string()));
            let len__843: i32 = temper_core::string::count_between( & val__842, 0usize, val__842.len());
            if Some(len__843) < Some(min__839) {
                t___9719 = true;
            } else {
                t___9719 = Some(len__843) > Some(max__840);
            }
            if t___9719 {
                t___17253 = field__838.sql_value();
                t___17254 = temper_core::int_to_string(min__839, None);
                t___17255 = temper_core::int_to_string(max__840, None);
                return__462 = self.add_error(t___17253.clone(), std::sync::Arc::new(format!("must be between {} and {} characters", t___17254.clone(), t___17255.clone())));
                break 'fn__841;
            }
            return__462 = Changeset::new(self.clone());
        }
        return return__462.clone();
    }
    pub fn validate_int(& self, field__845: SafeIdentifier) -> Changeset {
        let return__463: Changeset;
        let mut t___17243: std::sync::Arc<String>;
        let mut t___17246: std::sync::Arc<String>;
        'fn__846: {
            if ! self.0.is_valid {
                return__463 = Changeset::new(self.clone());
                break 'fn__846;
            }
            t___17243 = field__845.sql_value();
            let val__847: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___17243.clone(), std::sync::Arc::new("".to_string()));
            if val__847.is_empty() {
                return__463 = Changeset::new(self.clone());
                break 'fn__846;
            }
            let parseOk__848: bool;
            'ok___17391: {
                'orelse___2734: {
                    match temper_core::string::to_int( & val__847, None) {
                        Ok(x) => x,
                        _ => break 'orelse___2734
                    };
                    parseOk__848 = true;
                    break 'ok___17391;
                }
                parseOk__848 = false;
            }
            if ! parseOk__848 {
                t___17246 = field__845.sql_value();
                return__463 = self.add_error(t___17246.clone(), "must be an integer");
                break 'fn__846;
            }
            return__463 = Changeset::new(self.clone());
        }
        return return__463.clone();
    }
    pub fn validate_int64(& self, field__850: SafeIdentifier) -> Changeset {
        let return__464: Changeset;
        let mut t___17238: std::sync::Arc<String>;
        let mut t___17241: std::sync::Arc<String>;
        'fn__851: {
            if ! self.0.is_valid {
                return__464 = Changeset::new(self.clone());
                break 'fn__851;
            }
            t___17238 = field__850.sql_value();
            let val__852: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___17238.clone(), std::sync::Arc::new("".to_string()));
            if val__852.is_empty() {
                return__464 = Changeset::new(self.clone());
                break 'fn__851;
            }
            let parseOk__853: bool;
            'ok___17393: {
                'orelse___2735: {
                    match temper_core::string::to_int64( & val__852, None) {
                        Ok(x) => x,
                        _ => break 'orelse___2735
                    };
                    parseOk__853 = true;
                    break 'ok___17393;
                }
                parseOk__853 = false;
            }
            if ! parseOk__853 {
                t___17241 = field__850.sql_value();
                return__464 = self.add_error(t___17241.clone(), "must be a 64-bit integer");
                break 'fn__851;
            }
            return__464 = Changeset::new(self.clone());
        }
        return return__464.clone();
    }
    pub fn validate_float(& self, field__855: SafeIdentifier) -> Changeset {
        let return__465: Changeset;
        let mut t___17233: std::sync::Arc<String>;
        let mut t___17236: std::sync::Arc<String>;
        'fn__856: {
            if ! self.0.is_valid {
                return__465 = Changeset::new(self.clone());
                break 'fn__856;
            }
            t___17233 = field__855.sql_value();
            let val__857: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___17233.clone(), std::sync::Arc::new("".to_string()));
            if val__857.is_empty() {
                return__465 = Changeset::new(self.clone());
                break 'fn__856;
            }
            let parseOk__858: bool;
            'ok___17395: {
                'orelse___2736: {
                    match temper_core::string::to_float64( & val__857) {
                        Ok(x) => x,
                        _ => break 'orelse___2736
                    };
                    parseOk__858 = true;
                    break 'ok___17395;
                }
                parseOk__858 = false;
            }
            if ! parseOk__858 {
                t___17236 = field__855.sql_value();
                return__465 = self.add_error(t___17236.clone(), "must be a number");
                break 'fn__856;
            }
            return__465 = Changeset::new(self.clone());
        }
        return return__465.clone();
    }
    pub fn validate_bool(& self, field__860: SafeIdentifier) -> Changeset {
        let return__466: Changeset;
        let mut t___17228: std::sync::Arc<String>;
        let mut t___17231: std::sync::Arc<String>;
        let mut t___9687: bool;
        let mut t___9688: bool;
        let mut t___9690: bool;
        let mut t___9691: bool;
        let mut t___9693: bool;
        'fn__861: {
            if ! self.0.is_valid {
                return__466 = Changeset::new(self.clone());
                break 'fn__861;
            }
            t___17228 = field__860.sql_value();
            let val__862: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___17228.clone(), std::sync::Arc::new("".to_string()));
            if val__862.is_empty() {
                return__466 = Changeset::new(self.clone());
                break 'fn__861;
            }
            let isTrue__863: bool;
            if Some(val__862.as_str()) == Some("true") {
                isTrue__863 = true;
            } else {
                if Some(val__862.as_str()) == Some("1") {
                    t___9688 = true;
                } else {
                    if Some(val__862.as_str()) == Some("yes") {
                        t___9687 = true;
                    } else {
                        t___9687 = Some(val__862.as_str()) == Some("on");
                    }
                    t___9688 = t___9687;
                }
                isTrue__863 = t___9688;
            }
            let isFalse__864: bool;
            if Some(val__862.as_str()) == Some("false") {
                isFalse__864 = true;
            } else {
                if Some(val__862.as_str()) == Some("0") {
                    t___9691 = true;
                } else {
                    if Some(val__862.as_str()) == Some("no") {
                        t___9690 = true;
                    } else {
                        t___9690 = Some(val__862.as_str()) == Some("off");
                    }
                    t___9691 = t___9690;
                }
                isFalse__864 = t___9691;
            }
            if ! isTrue__863 {
                t___9693 = ! isFalse__864;
            } else {
                t___9693 = false;
            }
            if t___9693 {
                t___17231 = field__860.sql_value();
                return__466 = self.add_error(t___17231.clone(), "must be a boolean (true/false/1/0/yes/no/on/off)");
                break 'fn__861;
            }
            return__466 = Changeset::new(self.clone());
        }
        return return__466.clone();
    }
    pub fn put_change(& self, field__866: SafeIdentifier, value__867: impl temper_core::ToArcString) -> Changeset {
        let value__867 = value__867.to_arc_string();
        let mut t___17216: i32;
        let mb__869: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        let pairs__870: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        let mut i__871: i32 = 0;
        'loop___17523: loop {
            t___17216 = temper_core::ListedTrait::len( & pairs__870);
            if ! (Some(i__871) < Some(t___17216)) {
                break;
            }
            temper_core::MapBuilder::set( & mb__869, temper_core::ListedTrait::get( & pairs__870, i__871).key(), temper_core::ListedTrait::get( & pairs__870, i__871).value());
            i__871 = i__871.wrapping_add(1);
        }
        temper_core::MapBuilder::set( & mb__869, field__866.sql_value(), value__867.clone());
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__869), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn get_change(& self, field__873: SafeIdentifier) -> temper_core::Result<std::sync::Arc<String>> {
        let mut t___17210: std::sync::Arc<String> = field__873.sql_value();
        if ! temper_core::MappedTrait::has( & self.0.changes, t___17210.clone()) {
            return Err(temper_core::Error::new());
        }
        let mut t___17212: std::sync::Arc<String> = field__873.sql_value();
        return Ok(temper_core::MappedTrait::get_or( & self.0.changes, t___17212.clone(), std::sync::Arc::new("".to_string())));
    }
    pub fn delete_change(& self, field__876: SafeIdentifier) -> Changeset {
        let mut t___17197: i32;
        let mb__878: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        let pairs__879: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        let mut i__880: i32 = 0;
        'loop___17524: loop {
            t___17197 = temper_core::ListedTrait::len( & pairs__879);
            if ! (Some(i__880) < Some(t___17197)) {
                break;
            }
            if Some(temper_core::ListedTrait::get( & pairs__879, i__880).key().as_str()) != Some(field__876.sql_value().as_str()) {
                temper_core::MapBuilder::set( & mb__878, temper_core::ListedTrait::get( & pairs__879, i__880).key(), temper_core::ListedTrait::get( & pairs__879, i__880).value());
            }
            i__880 = i__880.wrapping_add(1);
        }
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__878), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn validate_inclusion(& self, field__882: SafeIdentifier, allowed__883: impl temper_core::ToList<std::sync::Arc<String>>) -> Changeset {
        let allowed__883 = allowed__883.to_list();
        let return__470: Changeset;
        let mut t___17187: std::sync::Arc<String>;
        let mut t___17189: std::sync::Arc<String>;
        let mut t___17193: std::sync::Arc<String>;
        'fn__884: {
            if ! self.0.is_valid {
                return__470 = Changeset::new(self.clone());
                break 'fn__884;
            }
            t___17187 = field__882.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___17187.clone()) {
                return__470 = Changeset::new(self.clone());
                break 'fn__884;
            }
            t___17189 = field__882.sql_value();
            let val__885: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___17189.clone(), std::sync::Arc::new("".to_string()));
            let mut found__886: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
            #[derive(Clone)]
            struct ClosureGroup___3 {
                val__885: std::sync::Arc<String>, found__886: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___3 {
                fn fn__17186(& self, a__887: impl temper_core::ToArcString) {
                    let a__887 = a__887.to_arc_string();
                    if Some(a__887.as_str()) == Some(self.val__885.as_str()) {
                        {
                            * self.found__886.write().unwrap() = true;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___3 {
                val__885: val__885.clone(), found__886: found__886.clone()
            };
            let fn__17186 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | a__887: std::sync::Arc<String> | closure_group.fn__17186(a__887))
            };
            temper_core::listed::list_for_each( & allowed__883, & ( * fn__17186.clone()));
            if ! temper_core::read_locked( & found__886) {
                t___17193 = field__882.sql_value();
                return__470 = self.add_error(t___17193.clone(), "is not included in the list");
                break 'fn__884;
            }
            return__470 = Changeset::new(self.clone());
        }
        return return__470.clone();
    }
    pub fn validate_exclusion(& self, field__889: SafeIdentifier, disallowed__890: impl temper_core::ToList<std::sync::Arc<String>>) -> Changeset {
        let disallowed__890 = disallowed__890.to_list();
        let return__471: Changeset;
        let mut t___17178: std::sync::Arc<String>;
        let mut t___17180: std::sync::Arc<String>;
        let mut t___17184: std::sync::Arc<String>;
        'fn__891: {
            if ! self.0.is_valid {
                return__471 = Changeset::new(self.clone());
                break 'fn__891;
            }
            t___17178 = field__889.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___17178.clone()) {
                return__471 = Changeset::new(self.clone());
                break 'fn__891;
            }
            t___17180 = field__889.sql_value();
            let val__892: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___17180.clone(), std::sync::Arc::new("".to_string()));
            let mut found__893: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
            #[derive(Clone)]
            struct ClosureGroup___4 {
                val__892: std::sync::Arc<String>, found__893: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___4 {
                fn fn__17177(& self, d__894: impl temper_core::ToArcString) {
                    let d__894 = d__894.to_arc_string();
                    if Some(d__894.as_str()) == Some(self.val__892.as_str()) {
                        {
                            * self.found__893.write().unwrap() = true;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___4 {
                val__892: val__892.clone(), found__893: found__893.clone()
            };
            let fn__17177 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | d__894: std::sync::Arc<String> | closure_group.fn__17177(d__894))
            };
            temper_core::listed::list_for_each( & disallowed__890, & ( * fn__17177.clone()));
            if temper_core::read_locked( & found__893) {
                t___17184 = field__889.sql_value();
                return__471 = self.add_error(t___17184.clone(), "is reserved");
                break 'fn__891;
            }
            return__471 = Changeset::new(self.clone());
        }
        return return__471.clone();
    }
    pub fn validate_number(& self, field__896: SafeIdentifier, opts__897: NumberValidationOpts) -> Changeset {
        let return__472: Changeset;
        let mut t___17151: std::sync::Arc<String>;
        let mut t___17153: std::sync::Arc<String>;
        let mut t___17155: std::sync::Arc<String>;
        let mut t___17158: std::sync::Arc<String>;
        let mut t___17159: std::sync::Arc<String>;
        let mut t___17162: std::sync::Arc<String>;
        let mut t___17163: std::sync::Arc<String>;
        let mut t___17166: std::sync::Arc<String>;
        let mut t___17167: std::sync::Arc<String>;
        let mut t___17170: std::sync::Arc<String>;
        let mut t___17171: std::sync::Arc<String>;
        let mut t___17174: std::sync::Arc<String>;
        let mut t___17175: std::sync::Arc<String>;
        let mut t___9621: f64;
        'fn__898: {
            if ! self.0.is_valid {
                return__472 = Changeset::new(self.clone());
                break 'fn__898;
            }
            t___17151 = field__896.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___17151.clone()) {
                return__472 = Changeset::new(self.clone());
                break 'fn__898;
            }
            t___17153 = field__896.sql_value();
            let val__899: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___17153.clone(), std::sync::Arc::new("".to_string()));
            let parseOk__900: bool;
            'ok___17400: {
                'orelse___2737: {
                    match temper_core::string::to_float64( & val__899) {
                        Ok(x) => x,
                        _ => break 'orelse___2737
                    };
                    parseOk__900 = true;
                    break 'ok___17400;
                }
                parseOk__900 = false;
            }
            if ! parseOk__900 {
                t___17155 = field__896.sql_value();
                return__472 = self.add_error(t___17155.clone(), "must be a number");
                break 'fn__898;
            }
            let num__901: f64;
            'ok___17401: {
                'orelse___2738: {
                    t___9621 = match temper_core::string::to_float64( & val__899) {
                        Ok(x) => x,
                        _ => break 'orelse___2738
                    };
                    num__901 = t___9621;
                    break 'ok___17401;
                }
                num__901 = 0.0f64;
            }
            let gt__902: Option<f64> = opts__897.greater_than();
            if ! gt__902.is_none() {
                let gt___2831: f64 = gt__902.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__901), Some(gt___2831)) > 0) {
                    t___17158 = field__896.sql_value();
                    t___17159 = temper_core::float64::to_string(gt___2831);
                    return__472 = self.add_error(t___17158.clone(), std::sync::Arc::new(format!("must be greater than {}", t___17159.clone())));
                    break 'fn__898;
                }
            }
            let lt__903: Option<f64> = opts__897.less_than();
            if ! lt__903.is_none() {
                let lt___2832: f64 = lt__903.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__901), Some(lt___2832)) < 0) {
                    t___17162 = field__896.sql_value();
                    t___17163 = temper_core::float64::to_string(lt___2832);
                    return__472 = self.add_error(t___17162.clone(), std::sync::Arc::new(format!("must be less than {}", t___17163.clone())));
                    break 'fn__898;
                }
            }
            let gte__904: Option<f64> = opts__897.greater_than_or_equal();
            if ! gte__904.is_none() {
                let gte___2833: f64 = gte__904.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__901), Some(gte___2833)) >= 0) {
                    t___17166 = field__896.sql_value();
                    t___17167 = temper_core::float64::to_string(gte___2833);
                    return__472 = self.add_error(t___17166.clone(), std::sync::Arc::new(format!("must be greater than or equal to {}", t___17167.clone())));
                    break 'fn__898;
                }
            }
            let lte__905: Option<f64> = opts__897.less_than_or_equal();
            if ! lte__905.is_none() {
                let lte___2834: f64 = lte__905.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__901), Some(lte___2834)) <= 0) {
                    t___17170 = field__896.sql_value();
                    t___17171 = temper_core::float64::to_string(lte___2834);
                    return__472 = self.add_error(t___17170.clone(), std::sync::Arc::new(format!("must be less than or equal to {}", t___17171.clone())));
                    break 'fn__898;
                }
            }
            let eq__906: Option<f64> = opts__897.equal_to();
            if ! eq__906.is_none() {
                let eq___2835: f64 = eq__906.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__901), Some(eq___2835)) == 0) {
                    t___17174 = field__896.sql_value();
                    t___17175 = temper_core::float64::to_string(eq___2835);
                    return__472 = self.add_error(t___17174.clone(), std::sync::Arc::new(format!("must be equal to {}", t___17175.clone())));
                    break 'fn__898;
                }
            }
            return__472 = Changeset::new(self.clone());
        }
        return return__472.clone();
    }
    pub fn validate_acceptance(& self, field__908: SafeIdentifier) -> Changeset {
        let return__473: Changeset;
        let mut t___17145: std::sync::Arc<String>;
        let mut t___17147: std::sync::Arc<String>;
        let mut t___17149: std::sync::Arc<String>;
        let mut t___9609: bool;
        let mut t___9610: bool;
        'fn__909: {
            if ! self.0.is_valid {
                return__473 = Changeset::new(self.clone());
                break 'fn__909;
            }
            t___17145 = field__908.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___17145.clone()) {
                return__473 = Changeset::new(self.clone());
                break 'fn__909;
            }
            t___17147 = field__908.sql_value();
            let val__910: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___17147.clone(), std::sync::Arc::new("".to_string()));
            let accepted__911: bool;
            if Some(val__910.as_str()) == Some("true") {
                accepted__911 = true;
            } else {
                if Some(val__910.as_str()) == Some("1") {
                    t___9610 = true;
                } else {
                    if Some(val__910.as_str()) == Some("yes") {
                        t___9609 = true;
                    } else {
                        t___9609 = Some(val__910.as_str()) == Some("on");
                    }
                    t___9610 = t___9609;
                }
                accepted__911 = t___9610;
            }
            if ! accepted__911 {
                t___17149 = field__908.sql_value();
                return__473 = self.add_error(t___17149.clone(), "must be accepted");
                break 'fn__909;
            }
            return__473 = Changeset::new(self.clone());
        }
        return return__473.clone();
    }
    pub fn validate_confirmation(& self, field__913: SafeIdentifier, confirmationField__914: SafeIdentifier) -> Changeset {
        let return__474: Changeset;
        let mut t___17137: std::sync::Arc<String>;
        let mut t___17139: std::sync::Arc<String>;
        let mut t___17141: std::sync::Arc<String>;
        let mut t___17143: std::sync::Arc<String>;
        'fn__915: {
            if ! self.0.is_valid {
                return__474 = Changeset::new(self.clone());
                break 'fn__915;
            }
            t___17137 = field__913.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___17137.clone()) {
                return__474 = Changeset::new(self.clone());
                break 'fn__915;
            }
            t___17139 = field__913.sql_value();
            let val__916: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___17139.clone(), std::sync::Arc::new("".to_string()));
            t___17141 = confirmationField__914.sql_value();
            let conf__917: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___17141.clone(), std::sync::Arc::new("".to_string()));
            if Some(val__916.as_str()) != Some(conf__917.as_str()) {
                t___17143 = confirmationField__914.sql_value();
                return__474 = self.add_error(t___17143.clone(), "does not match");
                break 'fn__915;
            }
            return__474 = Changeset::new(self.clone());
        }
        return return__474.clone();
    }
    pub fn validate_contains(& self, field__919: SafeIdentifier, substring__920: impl temper_core::ToArcString) -> Changeset {
        let substring__920 = substring__920.to_arc_string();
        let return__475: Changeset;
        let mut t___17129: std::sync::Arc<String>;
        let mut t___17131: std::sync::Arc<String>;
        let mut t___17135: std::sync::Arc<String>;
        'fn__921: {
            if ! self.0.is_valid {
                return__475 = Changeset::new(self.clone());
                break 'fn__921;
            }
            t___17129 = field__919.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___17129.clone()) {
                return__475 = Changeset::new(self.clone());
                break 'fn__921;
            }
            t___17131 = field__919.sql_value();
            let val__922: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___17131.clone(), std::sync::Arc::new("".to_string()));
            if ! temper_core::string::index_of( & val__922, substring__920.clone(), None).is_some() {
                t___17135 = field__919.sql_value();
                return__475 = self.add_error(t___17135.clone(), "must contain the given substring");
                break 'fn__921;
            }
            return__475 = Changeset::new(self.clone());
        }
        return return__475.clone();
    }
    pub fn validate_starts_with(& self, field__924: SafeIdentifier, prefix__925: impl temper_core::ToArcString) -> Changeset {
        let prefix__925 = prefix__925.to_arc_string();
        let return__476: Changeset;
        let mut t___17120: std::sync::Arc<String>;
        let mut t___17122: std::sync::Arc<String>;
        let mut t___17126: i32;
        let mut t___17127: std::sync::Arc<String>;
        'fn__926: {
            if ! self.0.is_valid {
                return__476 = Changeset::new(self.clone());
                break 'fn__926;
            }
            t___17120 = field__924.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___17120.clone()) {
                return__476 = Changeset::new(self.clone());
                break 'fn__926;
            }
            t___17122 = field__924.sql_value();
            let val__927: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___17122.clone(), std::sync::Arc::new("".to_string()));
            let idx__928: Option<usize> = temper_core::string::index_of( & val__927, prefix__925.clone(), None);
            let starts__929: bool;
            if idx__928.is_some() {
                t___17126 = temper_core::string::count_between( & val__927, 0usize, temper_core::string::cast_as_index(idx__928).unwrap());
                starts__929 = Some(t___17126) == Some(0);
            } else {
                starts__929 = false;
            }
            if ! starts__929 {
                t___17127 = field__924.sql_value();
                return__476 = self.add_error(t___17127.clone(), "must start with the given prefix");
                break 'fn__926;
            }
            return__476 = Changeset::new(self.clone());
        }
        return return__476.clone();
    }
    pub fn validate_ends_with(& self, field__931: SafeIdentifier, suffix__932: impl temper_core::ToArcString) -> Changeset {
        let suffix__932 = suffix__932.to_arc_string();
        let return__477: Changeset;
        let mut t___17100: std::sync::Arc<String>;
        let mut t___17102: std::sync::Arc<String>;
        let mut t___17107: usize;
        let mut t___17109: std::sync::Arc<String>;
        let mut t___17111: usize;
        let mut t___17112: bool;
        let mut t___17116: usize;
        let mut t___17117: usize;
        let mut t___17118: std::sync::Arc<String>;
        let mut t___9569: bool;
        'fn__933: {
            if ! self.0.is_valid {
                return__477 = Changeset::new(self.clone());
                break 'fn__933;
            }
            t___17100 = field__931.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___17100.clone()) {
                return__477 = Changeset::new(self.clone());
                break 'fn__933;
            }
            t___17102 = field__931.sql_value();
            let val__934: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___17102.clone(), std::sync::Arc::new("".to_string()));
            let valLen__935: i32 = temper_core::string::count_between( & val__934, 0usize, val__934.len());
            t___17107 = suffix__932.len();
            let suffixLen__936: i32 = temper_core::string::count_between( & suffix__932, 0usize, t___17107);
            if Some(valLen__935) < Some(suffixLen__936) {
                t___17109 = field__931.sql_value();
                return__477 = self.add_error(t___17109.clone(), "must end with the given suffix");
                break 'fn__933;
            }
            let skipCount__937: i32 = valLen__935.wrapping_sub(suffixLen__936);
            let mut strIdx__938: usize = 0usize;
            let mut i__939: i32 = 0;
            'loop___17530: while Some(i__939) < Some(skipCount__937) {
                t___17111 = temper_core::string::next( & val__934, strIdx__938);
                strIdx__938 = t___17111;
                i__939 = i__939.wrapping_add(1);
            }
            let mut sufIdx__940: usize = 0usize;
            let mut matches__941: bool = true;
            'loop___17531: loop {
                if matches__941 {
                    t___17112 = temper_core::string::has_index( & suffix__932, sufIdx__940);
                    t___9569 = t___17112;
                } else {
                    t___9569 = false;
                }
                if ! t___9569 {
                    break;
                }
                if ! temper_core::string::has_index( & val__934, strIdx__938) {
                    matches__941 = false;
                } else {
                    if Some(temper_core::string::get( & val__934, strIdx__938)) != Some(temper_core::string::get( & suffix__932, sufIdx__940)) {
                        matches__941 = false;
                    } else {
                        t___17116 = temper_core::string::next( & val__934, strIdx__938);
                        strIdx__938 = t___17116;
                        t___17117 = temper_core::string::next( & suffix__932, sufIdx__940);
                        sufIdx__940 = t___17117;
                    }
                }
            }
            if ! matches__941 {
                t___17118 = field__931.sql_value();
                return__477 = self.add_error(t___17118.clone(), "must end with the given suffix");
                break 'fn__933;
            }
            return__477 = Changeset::new(self.clone());
        }
        return return__477.clone();
    }
    fn parse_bool_sql_part(& self, val__943: impl temper_core::ToArcString) -> temper_core::Result<SqlBoolean> {
        let val__943 = val__943.to_arc_string();
        let return__478: SqlBoolean;
        let mut t___9547: bool;
        let mut t___9548: bool;
        let mut t___9549: bool;
        let mut t___9551: bool;
        let mut t___9552: bool;
        let mut t___9553: bool;
        'fn__944: {
            if Some(val__943.as_str()) == Some("true") {
                t___9549 = true;
            } else {
                if Some(val__943.as_str()) == Some("1") {
                    t___9548 = true;
                } else {
                    if Some(val__943.as_str()) == Some("yes") {
                        t___9547 = true;
                    } else {
                        t___9547 = Some(val__943.as_str()) == Some("on");
                    }
                    t___9548 = t___9547;
                }
                t___9549 = t___9548;
            }
            if t___9549 {
                return__478 = SqlBoolean::new(true);
                break 'fn__944;
            }
            if Some(val__943.as_str()) == Some("false") {
                t___9553 = true;
            } else {
                if Some(val__943.as_str()) == Some("0") {
                    t___9552 = true;
                } else {
                    if Some(val__943.as_str()) == Some("no") {
                        t___9551 = true;
                    } else {
                        t___9551 = Some(val__943.as_str()) == Some("off");
                    }
                    t___9552 = t___9551;
                }
                t___9553 = t___9552;
            }
            if t___9553 {
                return__478 = SqlBoolean::new(false);
                break 'fn__944;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__478.clone());
    }
    fn value_to_sql_part(& self, fieldDef__946: FieldDef, val__947: impl temper_core::ToArcString) -> temper_core::Result<SqlPart> {
        let val__947 = val__947.to_arc_string();
        let return__479: SqlPart;
        let mut t___9534: i32;
        let mut t___9537: i64;
        let mut t___9540: f64;
        let mut t___9545: temper_std::temporal::Date;
        'fn__948: {
            let ft__949: FieldType = fieldDef__946.field_type();
            if temper_core::is::<StringField>(ft__949.clone()) {
                return__479 = SqlPart::new(SqlString::new(val__947.clone()));
                break 'fn__948;
            }
            if temper_core::is::<IntField>(ft__949.clone()) {
                t___9534 = temper_core::string::to_int( & val__947, None) ? ;
                return__479 = SqlPart::new(SqlInt32::new(t___9534));
                break 'fn__948;
            }
            if temper_core::is::<Int64Field>(ft__949.clone()) {
                t___9537 = temper_core::string::to_int64( & val__947, None) ? ;
                return__479 = SqlPart::new(SqlInt64::new(t___9537));
                break 'fn__948;
            }
            if temper_core::is::<FloatField>(ft__949.clone()) {
                t___9540 = temper_core::string::to_float64( & val__947) ? ;
                return__479 = SqlPart::new(SqlFloat64::new(t___9540));
                break 'fn__948;
            }
            if temper_core::is::<BoolField>(ft__949.clone()) {
                return__479 = SqlPart::new(self.parse_bool_sql_part(val__947.clone()) ? );
                break 'fn__948;
            }
            if temper_core::is::<DateField>(ft__949.clone()) {
                t___9545 = temper_std::temporal::Date::from_iso_string(val__947.clone()) ? ;
                return__479 = SqlPart::new(SqlDate::new(t___9545.clone()));
                break 'fn__948;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__479.clone());
    }
    pub fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___17032: i32;
        let mut t___17039: std::sync::Arc<String>;
        let mut t___17044: i32;
        let mut t___17046: std::sync::Arc<String>;
        let mut t___17051: std::sync::Arc<String>;
        let mut t___17054: i32;
        let mut t___17060: std::sync::Arc<String>;
        let mut t___17080: i32;
        let mut t___9484: bool;
        let mut t___9485: bool;
        let mut t___9492: FieldDef;
        let mut t___9498: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let mut i__952: i32 = 0;
        'loop___17532: loop {
            'continue___17375: {
                t___17032 = temper_core::ListedTrait::len( & self.0.table_def.fields());
                if ! (Some(i__952) < Some(t___17032)) {
                    break 'loop___17532;
                }
                let f__953: FieldDef = temper_core::ListedTrait::get( & self.0.table_def.fields(), i__952);
                if f__953.r#virtual() {
                    break 'continue___17375;
                }
                let dv__954: Option<SqlPart> = f__953.default_value();
                if ! f__953.nullable() {
                    t___17039 = f__953.name().sql_value();
                    if ! temper_core::MappedTrait::has( & self.0.changes, t___17039.clone()) {
                        t___9484 = dv__954.is_none();
                    } else {
                        t___9484 = false;
                    }
                    t___9485 = t___9484;
                } else {
                    t___9485 = false;
                }
                if t___9485 {
                    return Err(temper_core::Error::new());
                }
            }
            i__952 = i__952.wrapping_add(1);
        }
        let colNames__955: temper_core::ListBuilder<std::sync::Arc<String>> = temper_core::listed::new_builder();
        let valParts__956: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        let pairs__957: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        let mut i__958: i32 = 0;
        'loop___17534: loop {
            'continue___17376: {
                t___17044 = temper_core::ListedTrait::len( & pairs__957);
                if ! (Some(i__958) < Some(t___17044)) {
                    break 'loop___17534;
                }
                let pair__959: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__957, i__958);
                t___17046 = pair__959.key();
                t___9492 = self.0.table_def.field(t___17046.clone()) ? ;
                let fd__960: FieldDef = t___9492.clone();
                if fd__960.r#virtual() {
                    break 'continue___17376;
                }
                temper_core::listed::add( & colNames__955, fd__960.name().sql_value(), None);
                t___17051 = pair__959.value();
                t___9498 = self.value_to_sql_part(fd__960.clone(), t___17051.clone()) ? ;
                temper_core::listed::add( & valParts__956, t___9498.clone(), None);
            }
            i__958 = i__958.wrapping_add(1);
        }
        let mut i__961: i32 = 0;
        'loop___17535: loop {
            'continue___17377: {
                t___17054 = temper_core::ListedTrait::len( & self.0.table_def.fields());
                if ! (Some(i__961) < Some(t___17054)) {
                    break 'loop___17535;
                }
                let f__962: FieldDef = temper_core::ListedTrait::get( & self.0.table_def.fields(), i__961);
                if f__962.r#virtual() {
                    break 'continue___17377;
                }
                let dv__963: Option<SqlPart> = f__962.default_value();
                if ! dv__963.is_none() {
                    let dv___2843: SqlPart = dv__963.clone().unwrap();
                    t___17060 = f__962.name().sql_value();
                    if ! temper_core::MappedTrait::has( & self.0.changes, t___17060.clone()) {
                        temper_core::listed::add( & colNames__955, f__962.name().sql_value(), None);
                        temper_core::listed::add( & valParts__956, dv___2843.clone(), None);
                    }
                }
            }
            i__961 = i__961.wrapping_add(1);
        }
        if Some(temper_core::ListedTrait::len( & valParts__956)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let b__964: SqlBuilder = SqlBuilder::new();
        b__964.append_safe("INSERT INTO ");
        b__964.append_safe(self.0.table_def.table_name().sql_value());
        b__964.append_safe(" (");
        let mut t___17073: temper_core::List<std::sync::Arc<String>> = temper_core::ListedTrait::to_list( & colNames__955);
        #[derive(Clone)]
        struct ClosureGroup___5 {}
        impl ClosureGroup___5 {
            fn fn__17030(& self, c__965: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let c__965 = c__965.to_arc_string();
                return c__965.clone();
            }
        }
        let closure_group = ClosureGroup___5 {};
        let fn__17030 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__965: std::sync::Arc<String> | closure_group.fn__17030(c__965))
        };
        b__964.append_safe(temper_core::listed::join( & t___17073, std::sync::Arc::new(", ".to_string()), & ( * fn__17030.clone())));
        b__964.append_safe(") VALUES (");
        b__964.append_part(temper_core::ListedTrait::get( & valParts__956, 0));
        let mut j__966: i32 = 1;
        'loop___17537: loop {
            t___17080 = temper_core::ListedTrait::len( & valParts__956);
            if ! (Some(j__966) < Some(t___17080)) {
                break;
            }
            b__964.append_safe(", ");
            b__964.append_part(temper_core::ListedTrait::get( & valParts__956, j__966));
            j__966 = j__966.wrapping_add(1);
        }
        b__964.append_safe(")");
        return Ok(b__964.accumulated());
    }
    pub fn to_update_sql(& self, id__968: i32) -> temper_core::Result<SqlFragment> {
        let mut t___17013: i32;
        let mut t___17015: std::sync::Arc<String>;
        let mut t___17022: std::sync::Arc<String>;
        let mut t___9459: FieldDef;
        let mut t___9466: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let pairs__970: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__970)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let b__971: SqlBuilder = SqlBuilder::new();
        b__971.append_safe("UPDATE ");
        b__971.append_safe(self.0.table_def.table_name().sql_value());
        b__971.append_safe(" SET ");
        let mut setCount__972: i32 = 0;
        let mut i__973: i32 = 0;
        'loop___17538: loop {
            'continue___17378: {
                t___17013 = temper_core::ListedTrait::len( & pairs__970);
                if ! (Some(i__973) < Some(t___17013)) {
                    break 'loop___17538;
                }
                let pair__974: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__970, i__973);
                t___17015 = pair__974.key();
                t___9459 = self.0.table_def.field(t___17015.clone()) ? ;
                let fd__975: FieldDef = t___9459.clone();
                if fd__975.r#virtual() {
                    break 'continue___17378;
                }
                if Some(setCount__972) > Some(0) {
                    b__971.append_safe(", ");
                }
                b__971.append_safe(fd__975.name().sql_value());
                b__971.append_safe(" = ");
                t___17022 = pair__974.value();
                t___9466 = self.value_to_sql_part(fd__975.clone(), t___17022.clone()) ? ;
                b__971.append_part(t___9466.clone());
                setCount__972 = setCount__972.wrapping_add(1);
            }
            i__973 = i__973.wrapping_add(1);
        }
        if Some(setCount__972) == Some(0) {
            return Err(temper_core::Error::new());
        }
        b__971.append_safe(" WHERE ");
        b__971.append_safe(self.0.table_def.pk_name());
        b__971.append_safe(" = ");
        b__971.append_int32(id__968);
        return Ok(b__971.accumulated());
    }
    pub fn new(_tableDef__977: TableDef, _params__978: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _changes__979: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _errors__980: impl temper_core::ToList<ChangesetError>, _isValid__981: bool) -> ChangesetImpl {
        let _errors__980 = _errors__980.to_list();
        let table_def;
        let params;
        let changes;
        let errors;
        let is_valid;
        table_def = _tableDef__977.clone();
        params = _params__978.clone();
        changes = _changes__979.clone();
        errors = _errors__980.clone();
        is_valid = _isValid__981;
        let selfish = ChangesetImpl(std::sync::Arc::new(ChangesetImplStruct {
                    table_def, params, changes, errors, is_valid
        }));
        return selfish;
    }
}
impl ChangesetTrait for ChangesetImpl {
    fn as_enum(& self) -> ChangesetEnum {
        ChangesetEnum::ChangesetImpl(self.clone())
    }
    fn clone_boxed(& self) -> Changeset {
        Changeset::new(self.clone())
    }
    fn table_def(& self) -> TableDef {
        self.table_def()
    }
    fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> {
        self.changes()
    }
    fn errors(& self) -> temper_core::List<ChangesetError> {
        self.errors()
    }
    fn is_valid(& self) -> bool {
        self.is_valid()
    }
    fn cast(& self, allowedFields__826: temper_core::List<SafeIdentifier>) -> Changeset {
        self.cast(allowedFields__826)
    }
    fn validate_required(& self, fields__832: temper_core::List<SafeIdentifier>) -> Changeset {
        self.validate_required(fields__832)
    }
    fn validate_length(& self, field__838: SafeIdentifier, min__839: i32, max__840: i32) -> Changeset {
        self.validate_length(field__838, min__839, max__840)
    }
    fn validate_int(& self, field__845: SafeIdentifier) -> Changeset {
        self.validate_int(field__845)
    }
    fn validate_int64(& self, field__850: SafeIdentifier) -> Changeset {
        self.validate_int64(field__850)
    }
    fn validate_float(& self, field__855: SafeIdentifier) -> Changeset {
        self.validate_float(field__855)
    }
    fn validate_bool(& self, field__860: SafeIdentifier) -> Changeset {
        self.validate_bool(field__860)
    }
    fn put_change(& self, field__866: SafeIdentifier, value__867: std::sync::Arc<String>) -> Changeset {
        self.put_change(field__866, value__867)
    }
    fn get_change(& self, field__873: SafeIdentifier) -> temper_core::Result<std::sync::Arc<String>> {
        self.get_change(field__873)
    }
    fn delete_change(& self, field__876: SafeIdentifier) -> Changeset {
        self.delete_change(field__876)
    }
    fn validate_inclusion(& self, field__882: SafeIdentifier, allowed__883: temper_core::List<std::sync::Arc<String>>) -> Changeset {
        self.validate_inclusion(field__882, allowed__883)
    }
    fn validate_exclusion(& self, field__889: SafeIdentifier, disallowed__890: temper_core::List<std::sync::Arc<String>>) -> Changeset {
        self.validate_exclusion(field__889, disallowed__890)
    }
    fn validate_number(& self, field__896: SafeIdentifier, opts__897: NumberValidationOpts) -> Changeset {
        self.validate_number(field__896, opts__897)
    }
    fn validate_acceptance(& self, field__908: SafeIdentifier) -> Changeset {
        self.validate_acceptance(field__908)
    }
    fn validate_confirmation(& self, field__913: SafeIdentifier, confirmationField__914: SafeIdentifier) -> Changeset {
        self.validate_confirmation(field__913, confirmationField__914)
    }
    fn validate_contains(& self, field__919: SafeIdentifier, substring__920: std::sync::Arc<String>) -> Changeset {
        self.validate_contains(field__919, substring__920)
    }
    fn validate_starts_with(& self, field__924: SafeIdentifier, prefix__925: std::sync::Arc<String>) -> Changeset {
        self.validate_starts_with(field__924, prefix__925)
    }
    fn validate_ends_with(& self, field__931: SafeIdentifier, suffix__932: std::sync::Arc<String>) -> Changeset {
        self.validate_ends_with(field__931, suffix__932)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        self.to_insert_sql()
    }
    fn to_update_sql(& self, id__968: i32) -> temper_core::Result<SqlFragment> {
        self.to_update_sql(id__968)
    }
}
temper_core::impl_any_value_trait!(ChangesetImpl, [Changeset]);
pub enum JoinTypeEnum {
    InnerJoin(InnerJoin), LeftJoin(LeftJoin), RightJoin(RightJoin), FullJoin(FullJoin), CrossJoin(CrossJoin)
}
pub trait JoinTypeTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> JoinTypeEnum;
    fn clone_boxed(& self) -> JoinType;
    fn keyword(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct JoinType(std::sync::Arc<dyn JoinTypeTrait>);
impl JoinType {
    pub fn new(selfish: impl JoinTypeTrait + 'static) -> JoinType {
        JoinType(std::sync::Arc::new(selfish))
    }
}
impl JoinTypeTrait for JoinType {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> JoinType {
        JoinTypeTrait::clone_boxed( & ( * self.0))
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        JoinTypeTrait::keyword( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(JoinType);
impl std::ops::Deref for JoinType {
    type Target = dyn JoinTypeTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct InnerJoinStruct {}
#[derive(Clone)]
pub struct InnerJoin(std::sync::Arc<InnerJoinStruct>);
impl InnerJoin {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("INNER JOIN".to_string());
    }
    pub fn new() -> InnerJoin {
        let selfish = InnerJoin(std::sync::Arc::new(InnerJoinStruct {}));
        return selfish;
    }
}
impl JoinTypeTrait for InnerJoin {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeEnum::InnerJoin(self.clone())
    }
    fn clone_boxed(& self) -> JoinType {
        JoinType::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(InnerJoin, [JoinType]);
struct LeftJoinStruct {}
#[derive(Clone)]
pub struct LeftJoin(std::sync::Arc<LeftJoinStruct>);
impl LeftJoin {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("LEFT JOIN".to_string());
    }
    pub fn new() -> LeftJoin {
        let selfish = LeftJoin(std::sync::Arc::new(LeftJoinStruct {}));
        return selfish;
    }
}
impl JoinTypeTrait for LeftJoin {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeEnum::LeftJoin(self.clone())
    }
    fn clone_boxed(& self) -> JoinType {
        JoinType::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(LeftJoin, [JoinType]);
struct RightJoinStruct {}
#[derive(Clone)]
pub struct RightJoin(std::sync::Arc<RightJoinStruct>);
impl RightJoin {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("RIGHT JOIN".to_string());
    }
    pub fn new() -> RightJoin {
        let selfish = RightJoin(std::sync::Arc::new(RightJoinStruct {}));
        return selfish;
    }
}
impl JoinTypeTrait for RightJoin {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeEnum::RightJoin(self.clone())
    }
    fn clone_boxed(& self) -> JoinType {
        JoinType::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(RightJoin, [JoinType]);
struct FullJoinStruct {}
#[derive(Clone)]
pub struct FullJoin(std::sync::Arc<FullJoinStruct>);
impl FullJoin {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("FULL OUTER JOIN".to_string());
    }
    pub fn new() -> FullJoin {
        let selfish = FullJoin(std::sync::Arc::new(FullJoinStruct {}));
        return selfish;
    }
}
impl JoinTypeTrait for FullJoin {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeEnum::FullJoin(self.clone())
    }
    fn clone_boxed(& self) -> JoinType {
        JoinType::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(FullJoin, [JoinType]);
struct CrossJoinStruct {}
#[derive(Clone)]
pub struct CrossJoin(std::sync::Arc<CrossJoinStruct>);
impl CrossJoin {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("CROSS JOIN".to_string());
    }
    pub fn new() -> CrossJoin {
        let selfish = CrossJoin(std::sync::Arc::new(CrossJoinStruct {}));
        return selfish;
    }
}
impl JoinTypeTrait for CrossJoin {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeEnum::CrossJoin(self.clone())
    }
    fn clone_boxed(& self) -> JoinType {
        JoinType::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(CrossJoin, [JoinType]);
struct JoinClauseStruct {
    join_type: JoinType, table: SafeIdentifier, on_condition: Option<SqlFragment>
}
#[derive(Clone)]
pub struct JoinClause(std::sync::Arc<JoinClauseStruct>);
#[derive(Clone)]
pub struct JoinClauseBuilder {
    pub join_type: JoinType, pub table: SafeIdentifier, pub on_condition: Option<SqlFragment>
}
impl JoinClauseBuilder {
    pub fn build(self) -> JoinClause {
        JoinClause::new(self.join_type, self.table, self.on_condition)
    }
}
impl JoinClause {
    pub fn new(joinType__1326: JoinType, table__1327: SafeIdentifier, onCondition__1328: Option<SqlFragment>) -> JoinClause {
        let join_type;
        let table;
        let on_condition;
        join_type = joinType__1326.clone();
        table = table__1327.clone();
        on_condition = onCondition__1328.clone();
        let selfish = JoinClause(std::sync::Arc::new(JoinClauseStruct {
                    join_type, table, on_condition
        }));
        return selfish;
    }
    pub fn join_type(& self) -> JoinType {
        return self.0.join_type.clone();
    }
    pub fn table(& self) -> SafeIdentifier {
        return self.0.table.clone();
    }
    pub fn on_condition(& self) -> Option<SqlFragment> {
        return self.0.on_condition.clone();
    }
}
temper_core::impl_any_value_trait!(JoinClause, []);
pub enum NullsPositionEnum {
    NullsFirst(NullsFirst), NullsLast(NullsLast)
}
pub trait NullsPositionTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> NullsPositionEnum;
    fn clone_boxed(& self) -> NullsPosition;
    fn keyword(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct NullsPosition(std::sync::Arc<dyn NullsPositionTrait>);
impl NullsPosition {
    pub fn new(selfish: impl NullsPositionTrait + 'static) -> NullsPosition {
        NullsPosition(std::sync::Arc::new(selfish))
    }
}
impl NullsPositionTrait for NullsPosition {
    fn as_enum(& self) -> NullsPositionEnum {
        NullsPositionTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> NullsPosition {
        NullsPositionTrait::clone_boxed( & ( * self.0))
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        NullsPositionTrait::keyword( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(NullsPosition);
impl std::ops::Deref for NullsPosition {
    type Target = dyn NullsPositionTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct NullsFirstStruct {}
#[derive(Clone)]
pub struct NullsFirst(std::sync::Arc<NullsFirstStruct>);
impl NullsFirst {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(" NULLS FIRST".to_string());
    }
    pub fn new() -> NullsFirst {
        let selfish = NullsFirst(std::sync::Arc::new(NullsFirstStruct {}));
        return selfish;
    }
}
impl NullsPositionTrait for NullsFirst {
    fn as_enum(& self) -> NullsPositionEnum {
        NullsPositionEnum::NullsFirst(self.clone())
    }
    fn clone_boxed(& self) -> NullsPosition {
        NullsPosition::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(NullsFirst, [NullsPosition]);
struct NullsLastStruct {}
#[derive(Clone)]
pub struct NullsLast(std::sync::Arc<NullsLastStruct>);
impl NullsLast {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(" NULLS LAST".to_string());
    }
    pub fn new() -> NullsLast {
        let selfish = NullsLast(std::sync::Arc::new(NullsLastStruct {}));
        return selfish;
    }
}
impl NullsPositionTrait for NullsLast {
    fn as_enum(& self) -> NullsPositionEnum {
        NullsPositionEnum::NullsLast(self.clone())
    }
    fn clone_boxed(& self) -> NullsPosition {
        NullsPosition::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(NullsLast, [NullsPosition]);
struct OrderClauseStruct {
    field: SafeIdentifier, ascending: bool, nulls_pos: Option<NullsPosition>
}
#[derive(Clone)]
pub struct OrderClause(std::sync::Arc<OrderClauseStruct>);
#[derive(Clone)]
pub struct OrderClauseBuilder {
    pub field: SafeIdentifier, pub ascending: bool, pub nulls_pos: Option<NullsPosition>
}
impl OrderClauseBuilder {
    pub fn build(self) -> OrderClause {
        OrderClause::new(self.field, self.ascending, self.nulls_pos)
    }
}
impl OrderClause {
    pub fn new(field__1341: SafeIdentifier, ascending__1342: bool, nullsPos__1343: Option<NullsPosition>) -> OrderClause {
        let field;
        let ascending;
        let nulls_pos;
        field = field__1341.clone();
        ascending = ascending__1342;
        nulls_pos = nullsPos__1343.clone();
        let selfish = OrderClause(std::sync::Arc::new(OrderClauseStruct {
                    field, ascending, nulls_pos
        }));
        return selfish;
    }
    pub fn field(& self) -> SafeIdentifier {
        return self.0.field.clone();
    }
    pub fn ascending(& self) -> bool {
        return self.0.ascending;
    }
    pub fn nulls_pos(& self) -> Option<NullsPosition> {
        return self.0.nulls_pos.clone();
    }
}
temper_core::impl_any_value_trait!(OrderClause, []);
pub enum LockModeEnum {
    ForUpdate(ForUpdate), ForShare(ForShare)
}
pub trait LockModeTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> LockModeEnum;
    fn clone_boxed(& self) -> LockMode;
    fn keyword(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct LockMode(std::sync::Arc<dyn LockModeTrait>);
impl LockMode {
    pub fn new(selfish: impl LockModeTrait + 'static) -> LockMode {
        LockMode(std::sync::Arc::new(selfish))
    }
}
impl LockModeTrait for LockMode {
    fn as_enum(& self) -> LockModeEnum {
        LockModeTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> LockMode {
        LockModeTrait::clone_boxed( & ( * self.0))
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        LockModeTrait::keyword( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(LockMode);
impl std::ops::Deref for LockMode {
    type Target = dyn LockModeTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct ForUpdateStruct {}
#[derive(Clone)]
pub struct ForUpdate(std::sync::Arc<ForUpdateStruct>);
impl ForUpdate {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(" FOR UPDATE".to_string());
    }
    pub fn new() -> ForUpdate {
        let selfish = ForUpdate(std::sync::Arc::new(ForUpdateStruct {}));
        return selfish;
    }
}
impl LockModeTrait for ForUpdate {
    fn as_enum(& self) -> LockModeEnum {
        LockModeEnum::ForUpdate(self.clone())
    }
    fn clone_boxed(& self) -> LockMode {
        LockMode::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(ForUpdate, [LockMode]);
struct ForShareStruct {}
#[derive(Clone)]
pub struct ForShare(std::sync::Arc<ForShareStruct>);
impl ForShare {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(" FOR SHARE".to_string());
    }
    pub fn new() -> ForShare {
        let selfish = ForShare(std::sync::Arc::new(ForShareStruct {}));
        return selfish;
    }
}
impl LockModeTrait for ForShare {
    fn as_enum(& self) -> LockModeEnum {
        LockModeEnum::ForShare(self.clone())
    }
    fn clone_boxed(& self) -> LockMode {
        LockMode::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(ForShare, [LockMode]);
pub enum WhereClauseEnum {
    AndCondition(AndCondition), OrCondition(OrCondition)
}
pub trait WhereClauseTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> WhereClauseEnum;
    fn clone_boxed(& self) -> WhereClause;
    fn condition(& self) -> SqlFragment;
    fn keyword(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct WhereClause(std::sync::Arc<dyn WhereClauseTrait>);
impl WhereClause {
    pub fn new(selfish: impl WhereClauseTrait + 'static) -> WhereClause {
        WhereClause(std::sync::Arc::new(selfish))
    }
}
impl WhereClauseTrait for WhereClause {
    fn as_enum(& self) -> WhereClauseEnum {
        WhereClauseTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> WhereClause {
        WhereClauseTrait::clone_boxed( & ( * self.0))
    }
    fn condition(& self) -> SqlFragment {
        WhereClauseTrait::condition( & ( * self.0))
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        WhereClauseTrait::keyword( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(WhereClause);
impl std::ops::Deref for WhereClause {
    type Target = dyn WhereClauseTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct AndConditionStruct {
    condition: SqlFragment
}
#[derive(Clone)]
pub struct AndCondition(std::sync::Arc<AndConditionStruct>);
impl AndCondition {
    pub fn condition(& self) -> SqlFragment {
        return self.0.condition.clone();
    }
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("AND".to_string());
    }
    pub fn new(_condition__1362: SqlFragment) -> AndCondition {
        let condition;
        condition = _condition__1362.clone();
        let selfish = AndCondition(std::sync::Arc::new(AndConditionStruct {
                    condition
        }));
        return selfish;
    }
}
impl WhereClauseTrait for AndCondition {
    fn as_enum(& self) -> WhereClauseEnum {
        WhereClauseEnum::AndCondition(self.clone())
    }
    fn clone_boxed(& self) -> WhereClause {
        WhereClause::new(self.clone())
    }
    fn condition(& self) -> SqlFragment {
        self.condition()
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(AndCondition, [WhereClause]);
struct OrConditionStruct {
    condition: SqlFragment
}
#[derive(Clone)]
pub struct OrCondition(std::sync::Arc<OrConditionStruct>);
impl OrCondition {
    pub fn condition(& self) -> SqlFragment {
        return self.0.condition.clone();
    }
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("OR".to_string());
    }
    pub fn new(_condition__1369: SqlFragment) -> OrCondition {
        let condition;
        condition = _condition__1369.clone();
        let selfish = OrCondition(std::sync::Arc::new(OrConditionStruct {
                    condition
        }));
        return selfish;
    }
}
impl WhereClauseTrait for OrCondition {
    fn as_enum(& self) -> WhereClauseEnum {
        WhereClauseEnum::OrCondition(self.clone())
    }
    fn clone_boxed(& self) -> WhereClause {
        WhereClause::new(self.clone())
    }
    fn condition(& self) -> SqlFragment {
        self.condition()
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(OrCondition, [WhereClause]);
struct QueryStruct {
    table_name: SafeIdentifier, conditions: temper_core::List<WhereClause>, selected_fields: temper_core::List<SafeIdentifier>, order_clauses: temper_core::List<OrderClause>, limit_val: Option<i32>, offset_val: Option<i32>, join_clauses: temper_core::List<JoinClause>, group_by_fields: temper_core::List<SafeIdentifier>, having_conditions: temper_core::List<WhereClause>, is_distinct: bool, select_exprs: temper_core::List<SqlFragment>, lock_mode: Option<LockMode>
}
#[derive(Clone)]
pub struct Query(std::sync::Arc<QueryStruct>);
#[derive(Clone)]
pub struct QueryBuilder {
    pub table_name: SafeIdentifier, pub conditions: temper_core::List<WhereClause>, pub selected_fields: temper_core::List<SafeIdentifier>, pub order_clauses: temper_core::List<OrderClause>, pub limit_val: Option<i32>, pub offset_val: Option<i32>, pub join_clauses: temper_core::List<JoinClause>, pub group_by_fields: temper_core::List<SafeIdentifier>, pub having_conditions: temper_core::List<WhereClause>, pub is_distinct: bool, pub select_exprs: temper_core::List<SqlFragment>, pub lock_mode: Option<LockMode>
}
impl QueryBuilder {
    pub fn build(self) -> Query {
        Query::new(self.table_name, self.conditions, self.selected_fields, self.order_clauses, self.limit_val, self.offset_val, self.join_clauses, self.group_by_fields, self.having_conditions, self.is_distinct, self.select_exprs, self.lock_mode)
    }
}
impl Query {
    pub fn r#where(& self, condition__1400: SqlFragment) -> Query {
        let nb__1402: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1402, WhereClause::new(AndCondition::new(condition__1400.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1402), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn or_where(& self, condition__1404: SqlFragment) -> Query {
        let nb__1406: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1406, WhereClause::new(OrCondition::new(condition__1404.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1406), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn where_null(& self, field__1408: SafeIdentifier) -> Query {
        let b__1410: SqlBuilder = SqlBuilder::new();
        b__1410.append_safe(field__1408.sql_value());
        b__1410.append_safe(" IS NULL");
        let mut t___15470: SqlFragment = b__1410.accumulated();
        return self.r#where(t___15470.clone());
    }
    pub fn where_not_null(& self, field__1412: SafeIdentifier) -> Query {
        let b__1414: SqlBuilder = SqlBuilder::new();
        b__1414.append_safe(field__1412.sql_value());
        b__1414.append_safe(" IS NOT NULL");
        let mut t___15464: SqlFragment = b__1414.accumulated();
        return self.r#where(t___15464.clone());
    }
    pub fn where_in(& self, field__1416: SafeIdentifier, values__1417: impl temper_core::ToList<SqlPart>) -> Query {
        let values__1417 = values__1417.to_list();
        let return__555: Query;
        let mut t___15445: SqlFragment;
        let mut t___15453: i32;
        let mut t___15458: SqlFragment;
        'fn__1418: {
            if temper_core::ListedTrait::is_empty( & values__1417) {
                let b__1419: SqlBuilder = SqlBuilder::new();
                b__1419.append_safe("1 = 0");
                t___15445 = b__1419.accumulated();
                return__555 = self.r#where(t___15445.clone());
                break 'fn__1418;
            }
            let b__1420: SqlBuilder = SqlBuilder::new();
            b__1420.append_safe(field__1416.sql_value());
            b__1420.append_safe(" IN (");
            b__1420.append_part(temper_core::ListedTrait::get( & values__1417, 0));
            let mut i__1421: i32 = 1;
            'loop___17549: loop {
                t___15453 = temper_core::ListedTrait::len( & values__1417);
                if ! (Some(i__1421) < Some(t___15453)) {
                    break;
                }
                b__1420.append_safe(", ");
                b__1420.append_part(temper_core::ListedTrait::get( & values__1417, i__1421));
                i__1421 = i__1421.wrapping_add(1);
            }
            b__1420.append_safe(")");
            t___15458 = b__1420.accumulated();
            return__555 = self.r#where(t___15458.clone());
        }
        return return__555.clone();
    }
    pub fn where_in_subquery(& self, field__1423: SafeIdentifier, sub__1424: Query) -> Query {
        let b__1426: SqlBuilder = SqlBuilder::new();
        b__1426.append_safe(field__1423.sql_value());
        b__1426.append_safe(" IN (");
        b__1426.append_fragment(sub__1424.to_sql());
        b__1426.append_safe(")");
        let mut t___15440: SqlFragment = b__1426.accumulated();
        return self.r#where(t___15440.clone());
    }
    pub fn where_not(& self, condition__1428: SqlFragment) -> Query {
        let b__1430: SqlBuilder = SqlBuilder::new();
        b__1430.append_safe("NOT (");
        b__1430.append_fragment(condition__1428.clone());
        b__1430.append_safe(")");
        let mut t___15431: SqlFragment = b__1430.accumulated();
        return self.r#where(t___15431.clone());
    }
    pub fn where_between(& self, field__1432: SafeIdentifier, low__1433: SqlPart, high__1434: SqlPart) -> Query {
        let b__1436: SqlBuilder = SqlBuilder::new();
        b__1436.append_safe(field__1432.sql_value());
        b__1436.append_safe(" BETWEEN ");
        b__1436.append_part(low__1433.clone());
        b__1436.append_safe(" AND ");
        b__1436.append_part(high__1434.clone());
        let mut t___15425: SqlFragment = b__1436.accumulated();
        return self.r#where(t___15425.clone());
    }
    pub fn where_like(& self, field__1438: SafeIdentifier, pattern__1439: impl temper_core::ToArcString) -> Query {
        let pattern__1439 = pattern__1439.to_arc_string();
        let b__1441: SqlBuilder = SqlBuilder::new();
        b__1441.append_safe(field__1438.sql_value());
        b__1441.append_safe(" LIKE ");
        b__1441.append_string(pattern__1439.clone());
        let mut t___15416: SqlFragment = b__1441.accumulated();
        return self.r#where(t___15416.clone());
    }
    pub fn where_i_like(& self, field__1443: SafeIdentifier, pattern__1444: impl temper_core::ToArcString) -> Query {
        let pattern__1444 = pattern__1444.to_arc_string();
        let b__1446: SqlBuilder = SqlBuilder::new();
        b__1446.append_safe(field__1443.sql_value());
        b__1446.append_safe(" ILIKE ");
        b__1446.append_string(pattern__1444.clone());
        let mut t___15409: SqlFragment = b__1446.accumulated();
        return self.r#where(t___15409.clone());
    }
    pub fn select(& self, fields__1448: impl temper_core::ToList<SafeIdentifier>) -> Query {
        let fields__1448 = fields__1448.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), fields__1448.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn select_expr(& self, exprs__1451: impl temper_core::ToList<SqlFragment>) -> Query {
        let exprs__1451 = exprs__1451.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, exprs__1451.clone(), self.0.lock_mode.clone());
    }
    pub fn order_by(& self, field__1454: SafeIdentifier, ascending__1455: bool) -> Query {
        let nb__1457: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__1457, OrderClause::new(field__1454.clone(), ascending__1455, None), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__1457), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn order_by_nulls(& self, field__1459: SafeIdentifier, ascending__1460: bool, nulls__1461: NullsPosition) -> Query {
        let nb__1463: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__1463, OrderClause::new(field__1459.clone(), ascending__1460, Some(nulls__1461.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__1463), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn limit(& self, n__1465: i32) -> temper_core::Result<Query> {
        if Some(n__1465) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), Some(n__1465), self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone()));
    }
    pub fn offset(& self, n__1468: i32) -> temper_core::Result<Query> {
        if Some(n__1468) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, Some(n__1468), self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone()));
    }
    pub fn join(& self, joinType__1471: JoinType, table__1472: SafeIdentifier, onCondition__1473: SqlFragment) -> Query {
        let nb__1475: temper_core::ListBuilder<JoinClause> = temper_core::ListedTrait::to_list_builder( & self.0.join_clauses);
        temper_core::listed::add( & nb__1475, JoinClause::new(joinType__1471.clone(), table__1472.clone(), Some(onCondition__1473.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, temper_core::ListedTrait::to_list( & nb__1475), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn inner_join(& self, table__1477: SafeIdentifier, onCondition__1478: SqlFragment) -> Query {
        let mut t___15371: InnerJoin = InnerJoin::new();
        return self.join(JoinType::new(t___15371.clone()), table__1477.clone(), onCondition__1478.clone());
    }
    pub fn left_join(& self, table__1481: SafeIdentifier, onCondition__1482: SqlFragment) -> Query {
        let mut t___15369: LeftJoin = LeftJoin::new();
        return self.join(JoinType::new(t___15369.clone()), table__1481.clone(), onCondition__1482.clone());
    }
    pub fn right_join(& self, table__1485: SafeIdentifier, onCondition__1486: SqlFragment) -> Query {
        let mut t___15367: RightJoin = RightJoin::new();
        return self.join(JoinType::new(t___15367.clone()), table__1485.clone(), onCondition__1486.clone());
    }
    pub fn full_join(& self, table__1489: SafeIdentifier, onCondition__1490: SqlFragment) -> Query {
        let mut t___15365: FullJoin = FullJoin::new();
        return self.join(JoinType::new(t___15365.clone()), table__1489.clone(), onCondition__1490.clone());
    }
    pub fn cross_join(& self, table__1493: SafeIdentifier) -> Query {
        let nb__1495: temper_core::ListBuilder<JoinClause> = temper_core::ListedTrait::to_list_builder( & self.0.join_clauses);
        temper_core::listed::add( & nb__1495, JoinClause::new(JoinType::new(CrossJoin::new()), table__1493.clone(), None), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, temper_core::ListedTrait::to_list( & nb__1495), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn group_by(& self, field__1497: SafeIdentifier) -> Query {
        let nb__1499: temper_core::ListBuilder<SafeIdentifier> = temper_core::ListedTrait::to_list_builder( & self.0.group_by_fields);
        temper_core::listed::add( & nb__1499, field__1497.clone(), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), temper_core::ListedTrait::to_list( & nb__1499), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn having(& self, condition__1501: SqlFragment) -> Query {
        let nb__1503: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.having_conditions);
        temper_core::listed::add( & nb__1503, WhereClause::new(AndCondition::new(condition__1501.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), temper_core::ListedTrait::to_list( & nb__1503), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn or_having(& self, condition__1505: SqlFragment) -> Query {
        let nb__1507: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.having_conditions);
        temper_core::listed::add( & nb__1507, WhereClause::new(OrCondition::new(condition__1505.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), temper_core::ListedTrait::to_list( & nb__1507), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn distinct(& self) -> Query {
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), true, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn lock(& self, mode__1511: LockMode) -> Query {
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), Some(mode__1511.clone()));
    }
    pub fn to_sql(& self) -> SqlFragment {
        let mut t___15287: i32;
        let b__1515: SqlBuilder = SqlBuilder::new();
        if self.0.is_distinct {
            b__1515.append_safe("SELECT DISTINCT ");
        } else {
            b__1515.append_safe("SELECT ");
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.select_exprs) {
            b__1515.append_fragment(temper_core::ListedTrait::get( & self.0.select_exprs, 0));
            let mut i__1516: i32 = 1;
            'loop___17586: loop {
                t___15287 = temper_core::ListedTrait::len( & self.0.select_exprs);
                if ! (Some(i__1516) < Some(t___15287)) {
                    break;
                }
                b__1515.append_safe(", ");
                b__1515.append_fragment(temper_core::ListedTrait::get( & self.0.select_exprs, i__1516));
                i__1516 = i__1516.wrapping_add(1);
            }
        } else {
            if temper_core::ListedTrait::is_empty( & self.0.selected_fields) {
                b__1515.append_safe("*");
            } else {
                #[derive(Clone)]
                struct ClosureGroup___6 {}
                impl ClosureGroup___6 {
                    fn fn__15280(& self, f__1517: SafeIdentifier) -> std::sync::Arc<String> {
                        return f__1517.sql_value();
                    }
                }
                let closure_group = ClosureGroup___6 {};
                let fn__15280 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | f__1517: SafeIdentifier | closure_group.fn__15280(f__1517))
                };
                b__1515.append_safe(temper_core::listed::join( & self.0.selected_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__15280.clone())));
            }
        }
        b__1515.append_safe(" FROM ");
        b__1515.append_safe(self.0.table_name.sql_value());
        renderJoins__706(b__1515.clone(), self.0.join_clauses.clone());
        renderWhere__705(b__1515.clone(), self.0.conditions.clone());
        renderGroupBy__707(b__1515.clone(), self.0.group_by_fields.clone());
        renderHaving__708(b__1515.clone(), self.0.having_conditions.clone());
        if ! temper_core::ListedTrait::is_empty( & self.0.order_clauses) {
            b__1515.append_safe(" ORDER BY ");
            let mut first__1518: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___7 {
                first__1518: std::sync::Arc<std::sync::RwLock<bool>>, b__1515: SqlBuilder
            }
            impl ClosureGroup___7 {
                fn fn__15279(& self, orc__1519: OrderClause) {
                    let mut t___15276: std::sync::Arc<String>;
                    let mut t___7962: std::sync::Arc<String>;
                    if ! temper_core::read_locked( & self.first__1518) {
                        self.b__1515.append_safe(", ");
                    }
                    {
                        * self.first__1518.write().unwrap() = false;
                    }
                    let mut t___15271: std::sync::Arc<String> = orc__1519.field().sql_value();
                    self.b__1515.append_safe(t___15271.clone());
                    if orc__1519.ascending() {
                        t___7962 = std::sync::Arc::new(" ASC".to_string());
                    } else {
                        t___7962 = std::sync::Arc::new(" DESC".to_string());
                    }
                    self.b__1515.append_safe(t___7962.clone());
                    let np__1520: Option<NullsPosition> = orc__1519.nulls_pos();
                    if ! np__1520.is_none() {
                        t___15276 = np__1520.clone().unwrap().keyword();
                        self.b__1515.append_safe(t___15276.clone());
                    }
                }
            }
            let closure_group = ClosureGroup___7 {
                first__1518: first__1518.clone(), b__1515: b__1515.clone()
            };
            let fn__15279 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | orc__1519: OrderClause | closure_group.fn__15279(orc__1519))
            };
            temper_core::listed::list_for_each( & self.0.order_clauses, & ( * fn__15279.clone()));
        }
        let lv__1521: Option<i32> = self.0.limit_val;
        if ! lv__1521.is_none() {
            let lv___2846: i32 = lv__1521.unwrap();
            b__1515.append_safe(" LIMIT ");
            b__1515.append_int32(lv___2846);
        }
        let ov__1522: Option<i32> = self.0.offset_val;
        if ! ov__1522.is_none() {
            let ov___2847: i32 = ov__1522.unwrap();
            b__1515.append_safe(" OFFSET ");
            b__1515.append_int32(ov___2847);
        }
        let lm__1523: Option<LockMode> = self.0.lock_mode.clone();
        if ! lm__1523.is_none() {
            b__1515.append_safe(lm__1523.clone().unwrap().keyword());
        }
        return b__1515.accumulated();
    }
    pub fn count_sql(& self) -> SqlFragment {
        let b__1526: SqlBuilder = SqlBuilder::new();
        b__1526.append_safe("SELECT COUNT(*) FROM ");
        b__1526.append_safe(self.0.table_name.sql_value());
        renderJoins__706(b__1526.clone(), self.0.join_clauses.clone());
        renderWhere__705(b__1526.clone(), self.0.conditions.clone());
        renderGroupBy__707(b__1526.clone(), self.0.group_by_fields.clone());
        renderHaving__708(b__1526.clone(), self.0.having_conditions.clone());
        return b__1526.accumulated();
    }
    pub fn safe_to_sql(& self, defaultLimit__1528: i32) -> temper_core::Result<SqlFragment> {
        let return__580: SqlFragment;
        let mut t___7946: Query;
        if Some(defaultLimit__1528) < Some(0) {
            return Err(temper_core::Error::new());
        }
        if ! self.0.limit_val.is_none() {
            return__580 = self.to_sql();
        } else {
            t___7946 = self.limit(defaultLimit__1528) ? ;
            return__580 = t___7946.to_sql();
        }
        return Ok(return__580.clone());
    }
    pub fn new(tableName__1531: SafeIdentifier, conditions__1532: impl temper_core::ToList<WhereClause>, selectedFields__1533: impl temper_core::ToList<SafeIdentifier>, orderClauses__1534: impl temper_core::ToList<OrderClause>, limitVal__1535: Option<i32>, offsetVal__1536: Option<i32>, joinClauses__1537: impl temper_core::ToList<JoinClause>, groupByFields__1538: impl temper_core::ToList<SafeIdentifier>, havingConditions__1539: impl temper_core::ToList<WhereClause>, isDistinct__1540: bool, selectExprs__1541: impl temper_core::ToList<SqlFragment>, lockMode__1542: Option<LockMode>) -> Query {
        let conditions__1532 = conditions__1532.to_list();
        let selectedFields__1533 = selectedFields__1533.to_list();
        let orderClauses__1534 = orderClauses__1534.to_list();
        let joinClauses__1537 = joinClauses__1537.to_list();
        let groupByFields__1538 = groupByFields__1538.to_list();
        let havingConditions__1539 = havingConditions__1539.to_list();
        let selectExprs__1541 = selectExprs__1541.to_list();
        let table_name;
        let conditions;
        let selected_fields;
        let order_clauses;
        let limit_val;
        let offset_val;
        let join_clauses;
        let group_by_fields;
        let having_conditions;
        let is_distinct;
        let select_exprs;
        let lock_mode;
        table_name = tableName__1531.clone();
        conditions = conditions__1532.clone();
        selected_fields = selectedFields__1533.clone();
        order_clauses = orderClauses__1534.clone();
        limit_val = limitVal__1535;
        offset_val = offsetVal__1536;
        join_clauses = joinClauses__1537.clone();
        group_by_fields = groupByFields__1538.clone();
        having_conditions = havingConditions__1539.clone();
        is_distinct = isDistinct__1540;
        select_exprs = selectExprs__1541.clone();
        lock_mode = lockMode__1542.clone();
        let selfish = Query(std::sync::Arc::new(QueryStruct {
                    table_name, conditions, selected_fields, order_clauses, limit_val, offset_val, join_clauses, group_by_fields, having_conditions, is_distinct, select_exprs, lock_mode
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn conditions(& self) -> temper_core::List<WhereClause> {
        return self.0.conditions.clone();
    }
    pub fn selected_fields(& self) -> temper_core::List<SafeIdentifier> {
        return self.0.selected_fields.clone();
    }
    pub fn order_clauses(& self) -> temper_core::List<OrderClause> {
        return self.0.order_clauses.clone();
    }
    pub fn limit_val(& self) -> Option<i32> {
        return self.0.limit_val;
    }
    pub fn offset_val(& self) -> Option<i32> {
        return self.0.offset_val;
    }
    pub fn join_clauses(& self) -> temper_core::List<JoinClause> {
        return self.0.join_clauses.clone();
    }
    pub fn group_by_fields(& self) -> temper_core::List<SafeIdentifier> {
        return self.0.group_by_fields.clone();
    }
    pub fn having_conditions(& self) -> temper_core::List<WhereClause> {
        return self.0.having_conditions.clone();
    }
    pub fn is_distinct(& self) -> bool {
        return self.0.is_distinct;
    }
    pub fn select_exprs(& self) -> temper_core::List<SqlFragment> {
        return self.0.select_exprs.clone();
    }
    pub fn lock_mode(& self) -> Option<LockMode> {
        return self.0.lock_mode.clone();
    }
}
temper_core::impl_any_value_trait!(Query, []);
struct SetClauseStruct {
    field: SafeIdentifier, value: SqlPart
}
#[derive(Clone)]
pub struct SetClause(std::sync::Arc<SetClauseStruct>);
#[derive(Clone)]
pub struct SetClauseBuilder {
    pub field: SafeIdentifier, pub value: SqlPart
}
impl SetClauseBuilder {
    pub fn build(self) -> SetClause {
        SetClause::new(self.field, self.value)
    }
}
impl SetClause {
    pub fn new(field__1592: SafeIdentifier, value__1593: SqlPart) -> SetClause {
        let field;
        let value;
        field = field__1592.clone();
        value = value__1593.clone();
        let selfish = SetClause(std::sync::Arc::new(SetClauseStruct {
                    field, value
        }));
        return selfish;
    }
    pub fn field(& self) -> SafeIdentifier {
        return self.0.field.clone();
    }
    pub fn value(& self) -> SqlPart {
        return self.0.value.clone();
    }
}
temper_core::impl_any_value_trait!(SetClause, []);
struct UpdateQueryStruct {
    table_name: SafeIdentifier, set_clauses: temper_core::List<SetClause>, conditions: temper_core::List<WhereClause>, limit_val: Option<i32>
}
#[derive(Clone)]
pub struct UpdateQuery(std::sync::Arc<UpdateQueryStruct>);
#[derive(Clone)]
pub struct UpdateQueryBuilder {
    pub table_name: SafeIdentifier, pub set_clauses: temper_core::List<SetClause>, pub conditions: temper_core::List<WhereClause>, pub limit_val: Option<i32>
}
impl UpdateQueryBuilder {
    pub fn build(self) -> UpdateQuery {
        UpdateQuery::new(self.table_name, self.set_clauses, self.conditions, self.limit_val)
    }
}
impl UpdateQuery {
    pub fn set(& self, field__1599: SafeIdentifier, value__1600: SqlPart) -> UpdateQuery {
        let nb__1602: temper_core::ListBuilder<SetClause> = temper_core::ListedTrait::to_list_builder( & self.0.set_clauses);
        temper_core::listed::add( & nb__1602, SetClause::new(field__1599.clone(), value__1600.clone()), None);
        return UpdateQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1602), self.0.conditions.clone(), self.0.limit_val);
    }
    pub fn r#where(& self, condition__1604: SqlFragment) -> UpdateQuery {
        let nb__1606: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1606, WhereClause::new(AndCondition::new(condition__1604.clone())), None);
        return UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), temper_core::ListedTrait::to_list( & nb__1606), self.0.limit_val);
    }
    pub fn or_where(& self, condition__1608: SqlFragment) -> UpdateQuery {
        let nb__1610: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1610, WhereClause::new(OrCondition::new(condition__1608.clone())), None);
        return UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), temper_core::ListedTrait::to_list( & nb__1610), self.0.limit_val);
    }
    pub fn limit(& self, n__1612: i32) -> temper_core::Result<UpdateQuery> {
        if Some(n__1612) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), self.0.conditions.clone(), Some(n__1612)));
    }
    pub fn to_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___15132: i32;
        if temper_core::ListedTrait::is_empty( & self.0.conditions) {
            return Err(temper_core::Error::new());
        }
        if temper_core::ListedTrait::is_empty( & self.0.set_clauses) {
            return Err(temper_core::Error::new());
        }
        let b__1616: SqlBuilder = SqlBuilder::new();
        b__1616.append_safe("UPDATE ");
        b__1616.append_safe(self.0.table_name.sql_value());
        b__1616.append_safe(" SET ");
        b__1616.append_safe(temper_core::ListedTrait::get( & self.0.set_clauses, 0).field().sql_value());
        b__1616.append_safe(" = ");
        b__1616.append_part(temper_core::ListedTrait::get( & self.0.set_clauses, 0).value());
        let mut i__1617: i32 = 1;
        'loop___17600: loop {
            t___15132 = temper_core::ListedTrait::len( & self.0.set_clauses);
            if ! (Some(i__1617) < Some(t___15132)) {
                break;
            }
            b__1616.append_safe(", ");
            b__1616.append_safe(temper_core::ListedTrait::get( & self.0.set_clauses, i__1617).field().sql_value());
            b__1616.append_safe(" = ");
            b__1616.append_part(temper_core::ListedTrait::get( & self.0.set_clauses, i__1617).value());
            i__1617 = i__1617.wrapping_add(1);
        }
        renderWhere__705(b__1616.clone(), self.0.conditions.clone());
        let lv__1618: Option<i32> = self.0.limit_val;
        if ! lv__1618.is_none() {
            let lv___2849: i32 = lv__1618.unwrap();
            b__1616.append_safe(" LIMIT ");
            b__1616.append_int32(lv___2849);
        }
        return Ok(b__1616.accumulated());
    }
    pub fn new(tableName__1620: SafeIdentifier, setClauses__1621: impl temper_core::ToList<SetClause>, conditions__1622: impl temper_core::ToList<WhereClause>, limitVal__1623: Option<i32>) -> UpdateQuery {
        let setClauses__1621 = setClauses__1621.to_list();
        let conditions__1622 = conditions__1622.to_list();
        let table_name;
        let set_clauses;
        let conditions;
        let limit_val;
        table_name = tableName__1620.clone();
        set_clauses = setClauses__1621.clone();
        conditions = conditions__1622.clone();
        limit_val = limitVal__1623;
        let selfish = UpdateQuery(std::sync::Arc::new(UpdateQueryStruct {
                    table_name, set_clauses, conditions, limit_val
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn set_clauses(& self) -> temper_core::List<SetClause> {
        return self.0.set_clauses.clone();
    }
    pub fn conditions(& self) -> temper_core::List<WhereClause> {
        return self.0.conditions.clone();
    }
    pub fn limit_val(& self) -> Option<i32> {
        return self.0.limit_val;
    }
}
temper_core::impl_any_value_trait!(UpdateQuery, []);
struct DeleteQueryStruct {
    table_name: SafeIdentifier, conditions: temper_core::List<WhereClause>, limit_val: Option<i32>
}
#[derive(Clone)]
pub struct DeleteQuery(std::sync::Arc<DeleteQueryStruct>);
#[derive(Clone)]
pub struct DeleteQueryBuilder {
    pub table_name: SafeIdentifier, pub conditions: temper_core::List<WhereClause>, pub limit_val: Option<i32>
}
impl DeleteQueryBuilder {
    pub fn build(self) -> DeleteQuery {
        DeleteQuery::new(self.table_name, self.conditions, self.limit_val)
    }
}
impl DeleteQuery {
    pub fn r#where(& self, condition__1628: SqlFragment) -> DeleteQuery {
        let nb__1630: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1630, WhereClause::new(AndCondition::new(condition__1628.clone())), None);
        return DeleteQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1630), self.0.limit_val);
    }
    pub fn or_where(& self, condition__1632: SqlFragment) -> DeleteQuery {
        let nb__1634: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1634, WhereClause::new(OrCondition::new(condition__1632.clone())), None);
        return DeleteQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1634), self.0.limit_val);
    }
    pub fn limit(& self, n__1636: i32) -> temper_core::Result<DeleteQuery> {
        if Some(n__1636) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(DeleteQuery::new(self.0.table_name.clone(), self.0.conditions.clone(), Some(n__1636)));
    }
    pub fn to_sql(& self) -> temper_core::Result<SqlFragment> {
        if temper_core::ListedTrait::is_empty( & self.0.conditions) {
            return Err(temper_core::Error::new());
        }
        let b__1640: SqlBuilder = SqlBuilder::new();
        b__1640.append_safe("DELETE FROM ");
        b__1640.append_safe(self.0.table_name.sql_value());
        renderWhere__705(b__1640.clone(), self.0.conditions.clone());
        let lv__1641: Option<i32> = self.0.limit_val;
        if ! lv__1641.is_none() {
            let lv___2850: i32 = lv__1641.unwrap();
            b__1640.append_safe(" LIMIT ");
            b__1640.append_int32(lv___2850);
        }
        return Ok(b__1640.accumulated());
    }
    pub fn new(tableName__1643: SafeIdentifier, conditions__1644: impl temper_core::ToList<WhereClause>, limitVal__1645: Option<i32>) -> DeleteQuery {
        let conditions__1644 = conditions__1644.to_list();
        let table_name;
        let conditions;
        let limit_val;
        table_name = tableName__1643.clone();
        conditions = conditions__1644.clone();
        limit_val = limitVal__1645;
        let selfish = DeleteQuery(std::sync::Arc::new(DeleteQueryStruct {
                    table_name, conditions, limit_val
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn conditions(& self) -> temper_core::List<WhereClause> {
        return self.0.conditions.clone();
    }
    pub fn limit_val(& self) -> Option<i32> {
        return self.0.limit_val;
    }
}
temper_core::impl_any_value_trait!(DeleteQuery, []);
pub enum SafeIdentifierEnum {
    ValidatedIdentifier(ValidatedIdentifier)
}
pub trait SafeIdentifierTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> SafeIdentifierEnum;
    fn clone_boxed(& self) -> SafeIdentifier;
    fn sql_value(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct SafeIdentifier(std::sync::Arc<dyn SafeIdentifierTrait>);
impl SafeIdentifier {
    pub fn new(selfish: impl SafeIdentifierTrait + 'static) -> SafeIdentifier {
        SafeIdentifier(std::sync::Arc::new(selfish))
    }
}
impl SafeIdentifierTrait for SafeIdentifier {
    fn as_enum(& self) -> SafeIdentifierEnum {
        SafeIdentifierTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> SafeIdentifier {
        SafeIdentifierTrait::clone_boxed( & ( * self.0))
    }
    fn sql_value(& self) -> std::sync::Arc<String> {
        SafeIdentifierTrait::sql_value( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(SafeIdentifier);
impl std::ops::Deref for SafeIdentifier {
    type Target = dyn SafeIdentifierTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct ValidatedIdentifierStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub (crate) struct ValidatedIdentifier(std::sync::Arc<ValidatedIdentifierStruct>);
impl ValidatedIdentifier {
    pub fn sql_value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
    pub fn new(_value__1905: impl temper_core::ToArcString) -> ValidatedIdentifier {
        let _value__1905 = _value__1905.to_arc_string();
        let value;
        value = _value__1905.clone();
        let selfish = ValidatedIdentifier(std::sync::Arc::new(ValidatedIdentifierStruct {
                    value
        }));
        return selfish;
    }
}
impl SafeIdentifierTrait for ValidatedIdentifier {
    fn as_enum(& self) -> SafeIdentifierEnum {
        SafeIdentifierEnum::ValidatedIdentifier(self.clone())
    }
    fn clone_boxed(& self) -> SafeIdentifier {
        SafeIdentifier::new(self.clone())
    }
    fn sql_value(& self) -> std::sync::Arc<String> {
        self.sql_value()
    }
}
temper_core::impl_any_value_trait!(ValidatedIdentifier, [SafeIdentifier]);
pub enum FieldTypeEnum {
    StringField(StringField), IntField(IntField), Int64Field(Int64Field), FloatField(FloatField), BoolField(BoolField), DateField(DateField)
}
pub trait FieldTypeTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> FieldTypeEnum;
    fn clone_boxed(& self) -> FieldType;
}
#[derive(Clone)]
pub struct FieldType(std::sync::Arc<dyn FieldTypeTrait>);
impl FieldType {
    pub fn new(selfish: impl FieldTypeTrait + 'static) -> FieldType {
        FieldType(std::sync::Arc::new(selfish))
    }
}
impl FieldTypeTrait for FieldType {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> FieldType {
        FieldTypeTrait::clone_boxed( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(FieldType);
impl std::ops::Deref for FieldType {
    type Target = dyn FieldTypeTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct StringFieldStruct {}
#[derive(Clone)]
pub struct StringField(std::sync::Arc<StringFieldStruct>);
impl StringField {
    pub fn new() -> StringField {
        let selfish = StringField(std::sync::Arc::new(StringFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for StringField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::StringField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(StringField, [FieldType]);
struct IntFieldStruct {}
#[derive(Clone)]
pub struct IntField(std::sync::Arc<IntFieldStruct>);
impl IntField {
    pub fn new() -> IntField {
        let selfish = IntField(std::sync::Arc::new(IntFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for IntField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::IntField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(IntField, [FieldType]);
struct Int64FieldStruct {}
#[derive(Clone)]
pub struct Int64Field(std::sync::Arc<Int64FieldStruct>);
impl Int64Field {
    pub fn new() -> Int64Field {
        let selfish = Int64Field(std::sync::Arc::new(Int64FieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for Int64Field {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::Int64Field(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Int64Field, [FieldType]);
struct FloatFieldStruct {}
#[derive(Clone)]
pub struct FloatField(std::sync::Arc<FloatFieldStruct>);
impl FloatField {
    pub fn new() -> FloatField {
        let selfish = FloatField(std::sync::Arc::new(FloatFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for FloatField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::FloatField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(FloatField, [FieldType]);
struct BoolFieldStruct {}
#[derive(Clone)]
pub struct BoolField(std::sync::Arc<BoolFieldStruct>);
impl BoolField {
    pub fn new() -> BoolField {
        let selfish = BoolField(std::sync::Arc::new(BoolFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for BoolField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::BoolField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(BoolField, [FieldType]);
struct DateFieldStruct {}
#[derive(Clone)]
pub struct DateField(std::sync::Arc<DateFieldStruct>);
impl DateField {
    pub fn new() -> DateField {
        let selfish = DateField(std::sync::Arc::new(DateFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for DateField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::DateField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(DateField, [FieldType]);
struct FieldDefStruct {
    name: SafeIdentifier, field_type: FieldType, nullable: bool, default_value: Option<SqlPart>, r#virtual: bool
}
#[derive(Clone)]
pub struct FieldDef(std::sync::Arc<FieldDefStruct>);
#[derive(Clone)]
pub struct FieldDefBuilder {
    pub name: SafeIdentifier, pub field_type: FieldType, pub nullable: bool, pub default_value: Option<SqlPart>, pub r#virtual: bool
}
impl FieldDefBuilder {
    pub fn build(self) -> FieldDef {
        FieldDef::new(self.name, self.field_type, self.nullable, self.default_value, self.r#virtual)
    }
}
impl FieldDef {
    pub fn new(name__1925: SafeIdentifier, fieldType__1926: FieldType, nullable__1927: bool, defaultValue__1928: Option<SqlPart>, virtual__1929: bool) -> FieldDef {
        let name;
        let field_type;
        let nullable;
        let default_value;
        let r#virtual;
        name = name__1925.clone();
        field_type = fieldType__1926.clone();
        nullable = nullable__1927;
        default_value = defaultValue__1928.clone();
        r#virtual = virtual__1929;
        let selfish = FieldDef(std::sync::Arc::new(FieldDefStruct {
                    name, field_type, nullable, default_value, r#virtual
        }));
        return selfish;
    }
    pub fn name(& self) -> SafeIdentifier {
        return self.0.name.clone();
    }
    pub fn field_type(& self) -> FieldType {
        return self.0.field_type.clone();
    }
    pub fn nullable(& self) -> bool {
        return self.0.nullable;
    }
    pub fn default_value(& self) -> Option<SqlPart> {
        return self.0.default_value.clone();
    }
    pub fn r#virtual(& self) -> bool {
        return self.0.r#virtual;
    }
}
temper_core::impl_any_value_trait!(FieldDef, []);
struct TableDefStruct {
    table_name: SafeIdentifier, fields: temper_core::List<FieldDef>, primary_key: Option<SafeIdentifier>
}
#[derive(Clone)]
pub struct TableDef(std::sync::Arc<TableDefStruct>);
#[derive(Clone)]
pub struct TableDefBuilder {
    pub table_name: SafeIdentifier, pub fields: temper_core::List<FieldDef>, pub primary_key: Option<SafeIdentifier>
}
impl TableDefBuilder {
    pub fn build(self) -> TableDef {
        TableDef::new(self.table_name, self.fields, self.primary_key)
    }
}
impl TableDef {
    pub fn field(& self, name__1934: impl temper_core::ToArcString) -> temper_core::Result<FieldDef> {
        let name__1934 = name__1934.to_arc_string();
        let return__646: FieldDef;
        'fn__1935: {
            let this__10148: temper_core::List<FieldDef> = self.0.fields.clone();
            let n__10149: i32 = temper_core::ListedTrait::len( & this__10148);
            let mut i__10150: i32 = 0;
            'loop___17611: while Some(i__10150) < Some(n__10149) {
                let el__10151: FieldDef = temper_core::ListedTrait::get( & this__10148, i__10150);
                i__10150 = i__10150.wrapping_add(1);
                let f__1936: FieldDef = el__10151.clone();
                if Some(f__1936.name().sql_value().as_str()) == Some(name__1934.as_str()) {
                    return__646 = f__1936.clone();
                    break 'fn__1935;
                }
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__646.clone());
    }
    pub fn pk_name(& self) -> std::sync::Arc<String> {
        let return__647: std::sync::Arc<String>;
        'fn__1938: {
            let pk__1939: Option<SafeIdentifier> = self.0.primary_key.clone();
            if ! pk__1939.is_none() {
                let pk___2830: SafeIdentifier = pk__1939.clone().unwrap();
                return__647 = pk___2830.sql_value();
                break 'fn__1938;
            }
            return__647 = std::sync::Arc::new("id".to_string());
        }
        return return__647.clone();
    }
    pub fn new(tableName__1941: SafeIdentifier, fields__1942: impl temper_core::ToList<FieldDef>, primaryKey__1943: Option<SafeIdentifier>) -> TableDef {
        let fields__1942 = fields__1942.to_list();
        let table_name;
        let fields;
        let primary_key;
        table_name = tableName__1941.clone();
        fields = fields__1942.clone();
        primary_key = primaryKey__1943.clone();
        let selfish = TableDef(std::sync::Arc::new(TableDefStruct {
                    table_name, fields, primary_key
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn fields(& self) -> temper_core::List<FieldDef> {
        return self.0.fields.clone();
    }
    pub fn primary_key(& self) -> Option<SafeIdentifier> {
        return self.0.primary_key.clone();
    }
}
temper_core::impl_any_value_trait!(TableDef, []);
struct SqlBuilderStruct {
    buffer: temper_core::ListBuilder<SqlPart>
}
#[derive(Clone)]
pub struct SqlBuilder(std::sync::Arc<SqlBuilderStruct>);
impl SqlBuilder {
    pub fn append_safe(& self, sqlSource__1986: impl temper_core::ToArcString) {
        let sqlSource__1986 = sqlSource__1986.to_arc_string();
        let mut t___17339: SqlSource = SqlSource::new(sqlSource__1986.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___17339.clone()), None);
    }
    pub fn append_fragment(& self, fragment__1989: SqlFragment) {
        let mut t___17337: temper_core::List<SqlPart> = fragment__1989.parts();
        temper_core::listed::add_all( & self.0.buffer, temper_core::ToListed::to_listed(t___17337.clone()), None);
    }
    pub fn append_part(& self, part__1992: SqlPart) {
        temper_core::listed::add( & self.0.buffer, part__1992.clone(), None);
    }
    pub fn append_part_list(& self, values__1995: impl temper_core::ToList<SqlPart>) {
        let values__1995 = values__1995.to_list();
        #[derive(Clone)]
        struct ClosureGroup___8 {
            this__379: SqlBuilder
        }
        impl ClosureGroup___8 {
            fn fn__17333(& self, x__1997: SqlPart) {
                self.this__379.append_part(x__1997.clone());
            }
        }
        let closure_group = ClosureGroup___8 {
            this__379: self.clone()
        };
        let fn__17333 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1997: SqlPart | closure_group.fn__17333(x__1997))
        };
        self.append_list(temper_core::ToListed::to_listed(values__1995.clone()), fn__17333.clone());
    }
    pub fn append_boolean(& self, value__1999: bool) {
        let mut t___17330: SqlBoolean = SqlBoolean::new(value__1999);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___17330.clone()), None);
    }
    pub fn append_boolean_list(& self, values__2002: impl temper_core::ToListed<bool>) {
        let values__2002 = values__2002.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___9 {
            this__381: SqlBuilder
        }
        impl ClosureGroup___9 {
            fn fn__17327(& self, x__2004: bool) {
                self.this__381.append_boolean(x__2004);
            }
        }
        let closure_group = ClosureGroup___9 {
            this__381: self.clone()
        };
        let fn__17327 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__2004: bool | closure_group.fn__17327(x__2004))
        };
        self.append_list(values__2002.clone(), fn__17327.clone());
    }
    pub fn append_date(& self, value__2006: temper_std::temporal::Date) {
        let mut t___17324: SqlDate = SqlDate::new(value__2006.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___17324.clone()), None);
    }
    pub fn append_date_list(& self, values__2009: impl temper_core::ToListed<temper_std::temporal::Date>) {
        let values__2009 = values__2009.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___10 {
            this__383: SqlBuilder
        }
        impl ClosureGroup___10 {
            fn fn__17321(& self, x__2011: temper_std::temporal::Date) {
                self.this__383.append_date(x__2011.clone());
            }
        }
        let closure_group = ClosureGroup___10 {
            this__383: self.clone()
        };
        let fn__17321 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__2011: temper_std::temporal::Date | closure_group.fn__17321(x__2011))
        };
        self.append_list(values__2009.clone(), fn__17321.clone());
    }
    pub fn append_float64(& self, value__2013: f64) {
        let mut t___17318: SqlFloat64 = SqlFloat64::new(value__2013);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___17318.clone()), None);
    }
    pub fn append_float64_list(& self, values__2016: impl temper_core::ToListed<f64>) {
        let values__2016 = values__2016.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___11 {
            this__385: SqlBuilder
        }
        impl ClosureGroup___11 {
            fn fn__17315(& self, x__2018: f64) {
                self.this__385.append_float64(x__2018);
            }
        }
        let closure_group = ClosureGroup___11 {
            this__385: self.clone()
        };
        let fn__17315 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__2018: f64 | closure_group.fn__17315(x__2018))
        };
        self.append_list(values__2016.clone(), fn__17315.clone());
    }
    pub fn append_int32(& self, value__2020: i32) {
        let mut t___17312: SqlInt32 = SqlInt32::new(value__2020);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___17312.clone()), None);
    }
    pub fn append_int32_list(& self, values__2023: impl temper_core::ToListed<i32>) {
        let values__2023 = values__2023.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___12 {
            this__387: SqlBuilder
        }
        impl ClosureGroup___12 {
            fn fn__17309(& self, x__2025: i32) {
                self.this__387.append_int32(x__2025);
            }
        }
        let closure_group = ClosureGroup___12 {
            this__387: self.clone()
        };
        let fn__17309 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__2025: i32 | closure_group.fn__17309(x__2025))
        };
        self.append_list(values__2023.clone(), fn__17309.clone());
    }
    pub fn append_int64(& self, value__2027: i64) {
        let mut t___17306: SqlInt64 = SqlInt64::new(value__2027);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___17306.clone()), None);
    }
    pub fn append_int64_list(& self, values__2030: impl temper_core::ToListed<i64>) {
        let values__2030 = values__2030.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___13 {
            this__389: SqlBuilder
        }
        impl ClosureGroup___13 {
            fn fn__17303(& self, x__2032: i64) {
                self.this__389.append_int64(x__2032);
            }
        }
        let closure_group = ClosureGroup___13 {
            this__389: self.clone()
        };
        let fn__17303 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__2032: i64 | closure_group.fn__17303(x__2032))
        };
        self.append_list(values__2030.clone(), fn__17303.clone());
    }
    pub fn append_string(& self, value__2034: impl temper_core::ToArcString) {
        let value__2034 = value__2034.to_arc_string();
        let mut t___17300: SqlString = SqlString::new(value__2034.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___17300.clone()), None);
    }
    pub fn append_string_list(& self, values__2037: impl temper_core::ToListed<std::sync::Arc<String>>) {
        let values__2037 = values__2037.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___14 {
            this__391: SqlBuilder
        }
        impl ClosureGroup___14 {
            fn fn__17297(& self, x__2039: impl temper_core::ToArcString) {
                let x__2039 = x__2039.to_arc_string();
                self.this__391.append_string(x__2039.clone());
            }
        }
        let closure_group = ClosureGroup___14 {
            this__391: self.clone()
        };
        let fn__17297 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__2039: std::sync::Arc<String> | closure_group.fn__17297(x__2039))
        };
        self.append_list(values__2037.clone(), fn__17297.clone());
    }
    fn append_list<T>(& self, values__2041: impl temper_core::ToListed<T>, appendValue__2042: std::sync::Arc<dyn Fn (T) + std::marker::Send + std::marker::Sync>) where T: Clone + std::marker::Send + std::marker::Sync + 'static {
        let values__2041 = values__2041.to_listed();
        let mut t___17292: i32;
        let mut t___17294: T;
        let mut i__2044: i32 = 0;
        'loop___17615: loop {
            t___17292 = temper_core::ListedTrait::len( & ( * values__2041));
            if ! (Some(i__2044) < Some(t___17292)) {
                break;
            }
            if Some(i__2044) > Some(0) {
                self.append_safe(", ");
            }
            t___17294 = temper_core::ListedTrait::get( & ( * values__2041), i__2044);
            appendValue__2042(t___17294.clone());
            i__2044 = i__2044.wrapping_add(1);
        }
    }
    pub fn accumulated(& self) -> SqlFragment {
        return SqlFragment::new(temper_core::ListedTrait::to_list( & self.0.buffer));
    }
    pub fn new() -> SqlBuilder {
        let buffer;
        let mut t___17289: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        buffer = t___17289.clone();
        let selfish = SqlBuilder(std::sync::Arc::new(SqlBuilderStruct {
                    buffer
        }));
        return selfish;
    }
}
temper_core::impl_any_value_trait!(SqlBuilder, []);
struct SqlFragmentStruct {
    parts: temper_core::List<SqlPart>
}
#[derive(Clone)]
pub struct SqlFragment(std::sync::Arc<SqlFragmentStruct>);
impl SqlFragment {
    pub fn to_source(& self) -> SqlSource {
        return SqlSource::new(self.to_string());
    }
    pub fn to_string(& self) -> std::sync::Arc<String> {
        let mut t___17363: i32;
        let builder__2056: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        let mut i__2057: i32 = 0;
        'loop___17616: loop {
            t___17363 = temper_core::ListedTrait::len( & self.0.parts);
            if ! (Some(i__2057) < Some(t___17363)) {
                break;
            }
            temper_core::ListedTrait::get( & self.0.parts, i__2057).format_to(builder__2056.clone());
            i__2057 = i__2057.wrapping_add(1);
        }
        return temper_core::string::builder::to_string( & builder__2056);
    }
    pub fn new(parts__2059: impl temper_core::ToList<SqlPart>) -> SqlFragment {
        let parts__2059 = parts__2059.to_list();
        let parts;
        parts = parts__2059.clone();
        let selfish = SqlFragment(std::sync::Arc::new(SqlFragmentStruct {
                    parts
        }));
        return selfish;
    }
    pub fn parts(& self) -> temper_core::List<SqlPart> {
        return self.0.parts.clone();
    }
}
temper_core::impl_any_value_trait!(SqlFragment, []);
pub enum SqlPartEnum {
    SqlSource(SqlSource), SqlBoolean(SqlBoolean), SqlString(SqlString), SqlInt32(SqlInt32), SqlInt64(SqlInt64), SqlFloat64(SqlFloat64), SqlDate(SqlDate), SqlDefault(SqlDefault)
}
pub trait SqlPartTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> SqlPartEnum;
    fn clone_boxed(& self) -> SqlPart;
    fn format_to(& self, builder__2061: std::sync::Arc<std::sync::RwLock<String>>);
}
#[derive(Clone)]
pub struct SqlPart(std::sync::Arc<dyn SqlPartTrait>);
impl SqlPart {
    pub fn new(selfish: impl SqlPartTrait + 'static) -> SqlPart {
        SqlPart(std::sync::Arc::new(selfish))
    }
}
impl SqlPartTrait for SqlPart {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPartTrait::clone_boxed( & ( * self.0))
    }
    fn format_to(& self, arg1: std::sync::Arc<std::sync::RwLock<String>>) -> () {
        SqlPartTrait::format_to( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(SqlPart);
impl std::ops::Deref for SqlPart {
    type Target = dyn SqlPartTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct SqlSourceStruct {
    source: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlSource(std::sync::Arc<SqlSourceStruct>);
impl SqlSource {
    pub fn format_to(& self, builder__2065: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__2065, self.0.source.clone());
    }
    pub fn new(source__2068: impl temper_core::ToArcString) -> SqlSource {
        let source__2068 = source__2068.to_arc_string();
        let source;
        source = source__2068.clone();
        let selfish = SqlSource(std::sync::Arc::new(SqlSourceStruct {
                    source
        }));
        return selfish;
    }
    pub fn source(& self) -> std::sync::Arc<String> {
        return self.0.source.clone();
    }
}
impl SqlPartTrait for SqlSource {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlSource(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__2065: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__2065)
    }
}
temper_core::impl_any_value_trait!(SqlSource, [SqlPart]);
struct SqlBooleanStruct {
    value: bool
}
#[derive(Clone)]
pub struct SqlBoolean(std::sync::Arc<SqlBooleanStruct>);
impl SqlBoolean {
    pub fn format_to(& self, builder__2071: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___9792: std::sync::Arc<String>;
        if self.0.value {
            t___9792 = std::sync::Arc::new("TRUE".to_string());
        } else {
            t___9792 = std::sync::Arc::new("FALSE".to_string());
        }
        temper_core::string::builder::append( & builder__2071, t___9792.clone());
    }
    pub fn new(value__2074: bool) -> SqlBoolean {
        let value;
        value = value__2074;
        let selfish = SqlBoolean(std::sync::Arc::new(SqlBooleanStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> bool {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlBoolean {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlBoolean(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__2071: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__2071)
    }
}
temper_core::impl_any_value_trait!(SqlBoolean, [SqlPart]);
struct SqlDateStruct {
    value: temper_std::temporal::Date
}
#[derive(Clone)]
pub struct SqlDate(std::sync::Arc<SqlDateStruct>);
impl SqlDate {
    pub fn format_to(& self, builder__2077: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__2077, "'");
        let mut t___17344: std::sync::Arc<String> = self.0.value.to_string();
        #[derive(Clone)]
        struct ClosureGroup___15 {
            builder__2077: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___15 {
            fn fn__17342(& self, c__2079: i32) {
                if Some(c__2079) == Some(39) {
                    temper_core::string::builder::append( & self.builder__2077, "''");
                } else {
                    'ok___17415: {
                        'orelse___2733: {
                            match temper_core::string::builder::append_code_point( & self.builder__2077, c__2079) {
                                Ok(x) => x,
                                _ => break 'orelse___2733
                            };
                            break 'ok___17415;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___15 {
            builder__2077: builder__2077.clone()
        };
        let fn__17342 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__2079: i32 | closure_group.fn__17342(c__2079))
        };
        temper_core::string::for_each( & t___17344, & ( * fn__17342.clone()));
        temper_core::string::builder::append( & builder__2077, "'");
    }
    pub fn new(value__2081: temper_std::temporal::Date) -> SqlDate {
        let value;
        value = value__2081.clone();
        let selfish = SqlDate(std::sync::Arc::new(SqlDateStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> temper_std::temporal::Date {
        return self.0.value.clone();
    }
}
impl SqlPartTrait for SqlDate {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlDate(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__2077: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__2077)
    }
}
temper_core::impl_any_value_trait!(SqlDate, [SqlPart]);
struct SqlFloat64Struct {
    value: f64
}
#[derive(Clone)]
pub struct SqlFloat64(std::sync::Arc<SqlFloat64Struct>);
impl SqlFloat64 {
    pub fn format_to(& self, builder__2084: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___9781: bool;
        let mut t___9782: bool;
        let s__2086: std::sync::Arc<String> = temper_core::float64::to_string(self.0.value);
        if Some(s__2086.as_str()) == Some("NaN") {
            t___9782 = true;
        } else {
            if Some(s__2086.as_str()) == Some("Infinity") {
                t___9781 = true;
            } else {
                t___9781 = Some(s__2086.as_str()) == Some("-Infinity");
            }
            t___9782 = t___9781;
        }
        if t___9782 {
            temper_core::string::builder::append( & builder__2084, "NULL");
        } else {
            temper_core::string::builder::append( & builder__2084, s__2086.clone());
        }
    }
    pub fn new(value__2088: f64) -> SqlFloat64 {
        let value;
        value = value__2088;
        let selfish = SqlFloat64(std::sync::Arc::new(SqlFloat64Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> f64 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlFloat64 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlFloat64(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__2084: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__2084)
    }
}
temper_core::impl_any_value_trait!(SqlFloat64, [SqlPart]);
struct SqlInt32Struct {
    value: i32
}
#[derive(Clone)]
pub struct SqlInt32(std::sync::Arc<SqlInt32Struct>);
impl SqlInt32 {
    pub fn format_to(& self, builder__2091: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___17353: std::sync::Arc<String> = temper_core::int_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__2091, t___17353.clone());
    }
    pub fn new(value__2094: i32) -> SqlInt32 {
        let value;
        value = value__2094;
        let selfish = SqlInt32(std::sync::Arc::new(SqlInt32Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> i32 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlInt32 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlInt32(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__2091: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__2091)
    }
}
temper_core::impl_any_value_trait!(SqlInt32, [SqlPart]);
struct SqlInt64Struct {
    value: i64
}
#[derive(Clone)]
pub struct SqlInt64(std::sync::Arc<SqlInt64Struct>);
impl SqlInt64 {
    pub fn format_to(& self, builder__2097: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___17351: std::sync::Arc<String> = temper_core::int64_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__2097, t___17351.clone());
    }
    pub fn new(value__2100: i64) -> SqlInt64 {
        let value;
        value = value__2100;
        let selfish = SqlInt64(std::sync::Arc::new(SqlInt64Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> i64 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlInt64 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlInt64(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__2097: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__2097)
    }
}
temper_core::impl_any_value_trait!(SqlInt64, [SqlPart]);
struct SqlDefaultStruct {}
#[derive(Clone)]
pub struct SqlDefault(std::sync::Arc<SqlDefaultStruct>);
impl SqlDefault {
    pub fn format_to(& self, builder__2102: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__2102, "DEFAULT");
    }
    pub fn new() -> SqlDefault {
        let selfish = SqlDefault(std::sync::Arc::new(SqlDefaultStruct {}));
        return selfish;
    }
}
impl SqlPartTrait for SqlDefault {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlDefault(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__2102: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__2102)
    }
}
temper_core::impl_any_value_trait!(SqlDefault, [SqlPart]);
struct SqlStringStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlString(std::sync::Arc<SqlStringStruct>);
impl SqlString {
    pub fn format_to(& self, builder__2107: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__2107, "'");
        #[derive(Clone)]
        struct ClosureGroup___16 {
            builder__2107: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___16 {
            fn fn__17356(& self, c__2109: i32) {
                if Some(c__2109) == Some(39) {
                    temper_core::string::builder::append( & self.builder__2107, "''");
                } else {
                    'ok___17419: {
                        'orelse___2732: {
                            match temper_core::string::builder::append_code_point( & self.builder__2107, c__2109) {
                                Ok(x) => x,
                                _ => break 'orelse___2732
                            };
                            break 'ok___17419;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___16 {
            builder__2107: builder__2107.clone()
        };
        let fn__17356 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__2109: i32 | closure_group.fn__17356(c__2109))
        };
        temper_core::string::for_each( & self.0.value, & ( * fn__17356.clone()));
        temper_core::string::builder::append( & builder__2107, "'");
    }
    pub fn new(value__2111: impl temper_core::ToArcString) -> SqlString {
        let value__2111 = value__2111.to_arc_string();
        let value;
        value = value__2111.clone();
        let selfish = SqlString(std::sync::Arc::new(SqlStringStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
}
impl SqlPartTrait for SqlString {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlString(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__2107: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__2107)
    }
}
temper_core::impl_any_value_trait!(SqlString, [SqlPart]);
pub fn changeset(tableDef__982: TableDef, params__983: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>) -> Changeset {
    let mut t___17003: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
    return Changeset::new(ChangesetImpl::new(tableDef__982.clone(), params__983.clone(), t___17003.clone(), [], true));
}
fn isIdentStart__710(c__1906: i32) -> bool {
    let return__624: bool;
    let mut t___9433: bool;
    let mut t___9434: bool;
    if Some(c__1906) >= Some(97) {
        t___9433 = Some(c__1906) <= Some(122);
    } else {
        t___9433 = false;
    }
    if t___9433 {
        return__624 = true;
    } else {
        if Some(c__1906) >= Some(65) {
            t___9434 = Some(c__1906) <= Some(90);
        } else {
            t___9434 = false;
        }
        if t___9434 {
            return__624 = true;
        } else {
            return__624 = Some(c__1906) == Some(95);
        }
    }
    return return__624;
}
fn isIdentPart__711(c__1908: i32) -> bool {
    let return__625: bool;
    if isIdentStart__710(c__1908) {
        return__625 = true;
    } else {
        if Some(c__1908) >= Some(48) {
            return__625 = Some(c__1908) <= Some(57);
        } else {
            return__625 = false;
        }
    }
    return return__625;
}
pub fn safe_identifier(name__1910: impl temper_core::ToArcString) -> temper_core::Result<SafeIdentifier> {
    let name__1910 = name__1910.to_arc_string();
    let mut t___17001: usize;
    if name__1910.is_empty() {
        return Err(temper_core::Error::new());
    }
    let mut idx__1912: usize = 0usize;
    if ! isIdentStart__710(temper_core::string::get( & name__1910, idx__1912)) {
        return Err(temper_core::Error::new());
    }
    let mut t___16998: usize = temper_core::string::next( & name__1910, idx__1912);
    idx__1912 = t___16998;
    'loop___17617: loop {
        if ! temper_core::string::has_index( & name__1910, idx__1912) {
            break;
        }
        if ! isIdentPart__711(temper_core::string::get( & name__1910, idx__1912)) {
            return Err(temper_core::Error::new());
        }
        t___17001 = temper_core::string::next( & name__1910, idx__1912);
        idx__1912 = t___17001;
    }
    return Ok(SafeIdentifier::new(ValidatedIdentifier::new(name__1910.clone())));
}
fn csid__703(name__985: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__985 = name__985.to_arc_string();
    let return__483: SafeIdentifier;
    let mut t___9421: SafeIdentifier;
    'ok___17421: {
        'orelse___2739: {
            t___9421 = match safe_identifier(name__985.clone()) {
                Ok(x) => x,
                _ => break 'orelse___2739
            };
            return__483 = t___9421.clone();
            break 'ok___17421;
        }
        return__483 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__483.clone();
}
fn userTable__704() -> TableDef {
    return TableDef::new(csid__703("users"), [FieldDef::new(csid__703("name"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("email"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("age"), FieldType::new(IntField::new()), true, None, false), FieldDef::new(csid__703("score"), FieldType::new(FloatField::new()), true, None, false), FieldDef::new(csid__703("active"), FieldType::new(BoolField::new()), true, None, false)], None);
}
pub fn timestamps() -> temper_core::Result<temper_core::List<FieldDef>> {
    let mut t___8692: SafeIdentifier;
    t___8692 = safe_identifier("inserted_at") ? ;
    let mut t___16100: FieldDef = FieldDef::new(t___8692.clone(), FieldType::new(DateField::new()), true, Some(SqlPart::new(SqlDefault::new())), false);
    let mut t___8696: SafeIdentifier;
    t___8696 = safe_identifier("updated_at") ? ;
    return Ok(std::sync::Arc::new(vec![t___16100.clone(), FieldDef::new(t___8696.clone(), FieldType::new(DateField::new()), true, Some(SqlPart::new(SqlDefault::new())), false)]));
}
pub fn delete_sql(tableDef__1301: TableDef, id__1302: i32) -> SqlFragment {
    let b__1304: SqlBuilder = SqlBuilder::new();
    b__1304.append_safe("DELETE FROM ");
    b__1304.append_safe(tableDef__1301.table_name().sql_value());
    b__1304.append_safe(" WHERE ");
    b__1304.append_safe(tableDef__1301.pk_name());
    b__1304.append_safe(" = ");
    b__1304.append_int32(id__1302);
    return b__1304.accumulated();
}
fn renderWhere__705(b__1370: SqlBuilder, conditions__1371: impl temper_core::ToList<WhereClause>) {
    let conditions__1371 = conditions__1371.to_list();
    let mut t___15522: SqlFragment;
    let mut t___15524: i32;
    let mut t___15527: std::sync::Arc<String>;
    let mut t___15531: SqlFragment;
    if ! temper_core::ListedTrait::is_empty( & conditions__1371) {
        b__1370.append_safe(" WHERE ");
        t___15522 = temper_core::ListedTrait::get( & conditions__1371, 0).condition();
        b__1370.append_fragment(t___15522.clone());
        let mut i__1373: i32 = 1;
        'loop___17618: loop {
            t___15524 = temper_core::ListedTrait::len( & conditions__1371);
            if ! (Some(i__1373) < Some(t___15524)) {
                break;
            }
            b__1370.append_safe(" ");
            t___15527 = temper_core::ListedTrait::get( & conditions__1371, i__1373).keyword();
            b__1370.append_safe(t___15527.clone());
            b__1370.append_safe(" ");
            t___15531 = temper_core::ListedTrait::get( & conditions__1371, i__1373).condition();
            b__1370.append_fragment(t___15531.clone());
            i__1373 = i__1373.wrapping_add(1);
        }
    }
}
fn renderJoins__706(b__1374: SqlBuilder, joinClauses__1375: impl temper_core::ToList<JoinClause>) {
    let joinClauses__1375 = joinClauses__1375.to_list();
    #[derive(Clone)]
    struct ClosureGroup___148 {
        b__1374: SqlBuilder
    }
    impl ClosureGroup___148 {
        fn fn__15516(& self, jc__1377: JoinClause) {
            self.b__1374.append_safe(" ");
            let mut t___15507: std::sync::Arc<String> = jc__1377.join_type().keyword();
            self.b__1374.append_safe(t___15507.clone());
            self.b__1374.append_safe(" ");
            let mut t___15511: std::sync::Arc<String> = jc__1377.table().sql_value();
            self.b__1374.append_safe(t___15511.clone());
            let oc__1378: Option<SqlFragment> = jc__1377.on_condition();
            if ! oc__1378.is_none() {
                let oc___2844: SqlFragment = oc__1378.clone().unwrap();
                self.b__1374.append_safe(" ON ");
                self.b__1374.append_fragment(oc___2844.clone());
            }
        }
    }
    let closure_group = ClosureGroup___148 {
        b__1374: b__1374.clone()
    };
    let fn__15516 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | jc__1377: JoinClause | closure_group.fn__15516(jc__1377))
    };
    temper_core::listed::list_for_each( & joinClauses__1375, & ( * fn__15516.clone()));
}
fn renderGroupBy__707(b__1379: SqlBuilder, groupByFields__1380: impl temper_core::ToList<SafeIdentifier>) {
    let groupByFields__1380 = groupByFields__1380.to_list();
    let mut t___15503: std::sync::Arc<String>;
    if ! temper_core::ListedTrait::is_empty( & groupByFields__1380) {
        b__1379.append_safe(" GROUP BY ");
        #[derive(Clone)]
        struct ClosureGroup___149 {}
        impl ClosureGroup___149 {
            fn fn__15499(& self, f__1382: SafeIdentifier) -> std::sync::Arc<String> {
                return f__1382.sql_value();
            }
        }
        let closure_group = ClosureGroup___149 {};
        let fn__15499 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__1382: SafeIdentifier | closure_group.fn__15499(f__1382))
        };
        t___15503 = temper_core::listed::join( & groupByFields__1380, std::sync::Arc::new(", ".to_string()), & ( * fn__15499.clone()));
        b__1379.append_safe(t___15503.clone());
    }
}
fn renderHaving__708(b__1383: SqlBuilder, havingConditions__1384: impl temper_core::ToList<WhereClause>) {
    let havingConditions__1384 = havingConditions__1384.to_list();
    let mut t___15487: SqlFragment;
    let mut t___15489: i32;
    let mut t___15492: std::sync::Arc<String>;
    let mut t___15496: SqlFragment;
    if ! temper_core::ListedTrait::is_empty( & havingConditions__1384) {
        b__1383.append_safe(" HAVING ");
        t___15487 = temper_core::ListedTrait::get( & havingConditions__1384, 0).condition();
        b__1383.append_fragment(t___15487.clone());
        let mut i__1386: i32 = 1;
        'loop___17620: loop {
            t___15489 = temper_core::ListedTrait::len( & havingConditions__1384);
            if ! (Some(i__1386) < Some(t___15489)) {
                break;
            }
            b__1383.append_safe(" ");
            t___15492 = temper_core::ListedTrait::get( & havingConditions__1384, i__1386).keyword();
            b__1383.append_safe(t___15492.clone());
            b__1383.append_safe(" ");
            t___15496 = temper_core::ListedTrait::get( & havingConditions__1384, i__1386).condition();
            b__1383.append_fragment(t___15496.clone());
            i__1386 = i__1386.wrapping_add(1);
        }
    }
}
pub fn from(tableName__1543: SafeIdentifier) -> Query {
    return Query::new(tableName__1543.clone(), [], [], [], None, None, [], [], [], false, [], None);
}
pub fn col(table__1545: SafeIdentifier, column__1546: SafeIdentifier) -> SqlFragment {
    let b__1548: SqlBuilder = SqlBuilder::new();
    b__1548.append_safe(table__1545.sql_value());
    b__1548.append_safe(".");
    b__1548.append_safe(column__1546.sql_value());
    return b__1548.accumulated();
}
pub fn count_all() -> SqlFragment {
    let b__1550: SqlBuilder = SqlBuilder::new();
    b__1550.append_safe("COUNT(*)");
    return b__1550.accumulated();
}
pub fn count_col(field__1551: SafeIdentifier) -> SqlFragment {
    let b__1553: SqlBuilder = SqlBuilder::new();
    b__1553.append_safe("COUNT(");
    b__1553.append_safe(field__1551.sql_value());
    b__1553.append_safe(")");
    return b__1553.accumulated();
}
pub fn sum_col(field__1554: SafeIdentifier) -> SqlFragment {
    let b__1556: SqlBuilder = SqlBuilder::new();
    b__1556.append_safe("SUM(");
    b__1556.append_safe(field__1554.sql_value());
    b__1556.append_safe(")");
    return b__1556.accumulated();
}
pub fn avg_col(field__1557: SafeIdentifier) -> SqlFragment {
    let b__1559: SqlBuilder = SqlBuilder::new();
    b__1559.append_safe("AVG(");
    b__1559.append_safe(field__1557.sql_value());
    b__1559.append_safe(")");
    return b__1559.accumulated();
}
pub fn min_col(field__1560: SafeIdentifier) -> SqlFragment {
    let b__1562: SqlBuilder = SqlBuilder::new();
    b__1562.append_safe("MIN(");
    b__1562.append_safe(field__1560.sql_value());
    b__1562.append_safe(")");
    return b__1562.accumulated();
}
pub fn max_col(field__1563: SafeIdentifier) -> SqlFragment {
    let b__1565: SqlBuilder = SqlBuilder::new();
    b__1565.append_safe("MAX(");
    b__1565.append_safe(field__1563.sql_value());
    b__1565.append_safe(")");
    return b__1565.accumulated();
}
pub fn union_sql(a__1566: Query, b__1567: Query) -> SqlFragment {
    let sb__1569: SqlBuilder = SqlBuilder::new();
    sb__1569.append_safe("(");
    sb__1569.append_fragment(a__1566.to_sql());
    sb__1569.append_safe(") UNION (");
    sb__1569.append_fragment(b__1567.to_sql());
    sb__1569.append_safe(")");
    return sb__1569.accumulated();
}
pub fn union_all_sql(a__1570: Query, b__1571: Query) -> SqlFragment {
    let sb__1573: SqlBuilder = SqlBuilder::new();
    sb__1573.append_safe("(");
    sb__1573.append_fragment(a__1570.to_sql());
    sb__1573.append_safe(") UNION ALL (");
    sb__1573.append_fragment(b__1571.to_sql());
    sb__1573.append_safe(")");
    return sb__1573.accumulated();
}
pub fn intersect_sql(a__1574: Query, b__1575: Query) -> SqlFragment {
    let sb__1577: SqlBuilder = SqlBuilder::new();
    sb__1577.append_safe("(");
    sb__1577.append_fragment(a__1574.to_sql());
    sb__1577.append_safe(") INTERSECT (");
    sb__1577.append_fragment(b__1575.to_sql());
    sb__1577.append_safe(")");
    return sb__1577.accumulated();
}
pub fn except_sql(a__1578: Query, b__1579: Query) -> SqlFragment {
    let sb__1581: SqlBuilder = SqlBuilder::new();
    sb__1581.append_safe("(");
    sb__1581.append_fragment(a__1578.to_sql());
    sb__1581.append_safe(") EXCEPT (");
    sb__1581.append_fragment(b__1579.to_sql());
    sb__1581.append_safe(")");
    return sb__1581.accumulated();
}
pub fn subquery(q__1582: Query, alias__1583: SafeIdentifier) -> SqlFragment {
    let b__1585: SqlBuilder = SqlBuilder::new();
    b__1585.append_safe("(");
    b__1585.append_fragment(q__1582.to_sql());
    b__1585.append_safe(") AS ");
    b__1585.append_safe(alias__1583.sql_value());
    return b__1585.accumulated();
}
pub fn exists_sql(q__1586: Query) -> SqlFragment {
    let b__1588: SqlBuilder = SqlBuilder::new();
    b__1588.append_safe("EXISTS (");
    b__1588.append_fragment(q__1586.to_sql());
    b__1588.append_safe(")");
    return b__1588.accumulated();
}
pub fn update(tableName__1646: SafeIdentifier) -> UpdateQuery {
    return UpdateQuery::new(tableName__1646.clone(), [], [], None);
}
pub fn delete_from(tableName__1648: SafeIdentifier) -> DeleteQuery {
    return DeleteQuery::new(tableName__1648.clone(), [], None);
}
fn sid__709(name__1650: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__1650 = name__1650.to_arc_string();
    let return__617: SafeIdentifier;
    let mut t___7743: SafeIdentifier;
    'ok___17445: {
        'orelse___2762: {
            t___7743 = match safe_identifier(name__1650.clone()) {
                Ok(x) => x,
                _ => break 'orelse___2762
            };
            return__617 = t___7743.clone();
            break 'ok___17445;
        }
        return__617 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__617.clone();
}
#[cfg(test)]
mod tests {
    #[test]
    fn castWhitelistsAllowedFields__2272() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let params__989: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string())), (std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("true".to_string()))]);
        let mut t___16959: TableDef = userTable__704();
        let mut t___16960: SafeIdentifier = csid__703("name");
        let mut t___16961: SafeIdentifier = csid__703("email");
        let cs__990: Changeset = changeset(t___16959.clone(), params__989.clone()).cast(std::sync::Arc::new(vec![t___16960.clone(), t___16961.clone()]));
        let mut t___16964: bool = temper_core::MappedTrait::has( & cs__990.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___17 {}
        impl ClosureGroup___17 {
            fn fn__16954(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___17 {};
        let fn__16954 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16954())
        };
        test___32.assert(t___16964, fn__16954.clone());
        let mut t___16968: bool = temper_core::MappedTrait::has( & cs__990.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___18 {}
        impl ClosureGroup___18 {
            fn fn__16953(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___18 {};
        let fn__16953 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16953())
        };
        test___32.assert(t___16968, fn__16953.clone());
        let mut t___16974: bool = ! temper_core::MappedTrait::has( & cs__990.changes(), std::sync::Arc::new("admin".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___19 {}
        impl ClosureGroup___19 {
            fn fn__16952(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("admin must be dropped (not in whitelist)".to_string());
            }
        }
        let closure_group = ClosureGroup___19 {};
        let fn__16952 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16952())
        };
        test___32.assert(t___16974, fn__16952.clone());
        let mut t___16976: bool = cs__990.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___20 {}
        impl ClosureGroup___20 {
            fn fn__16951(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___20 {};
        let fn__16951 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16951())
        };
        test___32.assert(t___16976, fn__16951.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn castIsReplacingNotAdditiveSecondCallResetsWhitelist__2273() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        let params__992: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___16937: TableDef = userTable__704();
        let mut t___16938: SafeIdentifier = csid__703("name");
        let cs__993: Changeset = changeset(t___16937.clone(), params__992.clone()).cast(std::sync::Arc::new(vec![t___16938.clone()])).cast(std::sync::Arc::new(vec![csid__703("email")]));
        let mut t___16945: bool = ! temper_core::MappedTrait::has( & cs__993.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___21 {}
        impl ClosureGroup___21 {
            fn fn__16933(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name must be excluded by second cast".to_string());
            }
        }
        let closure_group = ClosureGroup___21 {};
        let fn__16933 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16933())
        };
        test___33.assert(t___16945, fn__16933.clone());
        let mut t___16948: bool = temper_core::MappedTrait::has( & cs__993.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___22 {}
        impl ClosureGroup___22 {
            fn fn__16932(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be present".to_string());
            }
        }
        let closure_group = ClosureGroup___22 {};
        let fn__16932 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16932())
        };
        test___33.assert(t___16948, fn__16932.clone());
        test___33.soft_fail_to_hard()
    }
    #[test]
    fn castIgnoresEmptyStringValues__2274() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___34 = temper_std::testing::Test::new();
        let params__995: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bob@example.com".to_string()))]);
        let mut t___16919: TableDef = userTable__704();
        let mut t___16920: SafeIdentifier = csid__703("name");
        let mut t___16921: SafeIdentifier = csid__703("email");
        let cs__996: Changeset = changeset(t___16919.clone(), params__995.clone()).cast(std::sync::Arc::new(vec![t___16920.clone(), t___16921.clone()]));
        let mut t___16926: bool = ! temper_core::MappedTrait::has( & cs__996.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___23 {}
        impl ClosureGroup___23 {
            fn fn__16915(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty name should not be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___23 {};
        let fn__16915 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16915())
        };
        test___34.assert(t___16926, fn__16915.clone());
        let mut t___16929: bool = temper_core::MappedTrait::has( & cs__996.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___24 {}
        impl ClosureGroup___24 {
            fn fn__16914(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___24 {};
        let fn__16914 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16914())
        };
        test___34.assert(t___16929, fn__16914.clone());
        test___34.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredPassesWhenFieldPresent__2275() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___35 = temper_std::testing::Test::new();
        let params__998: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___16901: TableDef = userTable__704();
        let mut t___16902: SafeIdentifier = csid__703("name");
        let cs__999: Changeset = changeset(t___16901.clone(), params__998.clone()).cast(std::sync::Arc::new(vec![t___16902.clone()])).validate_required(std::sync::Arc::new(vec![csid__703("name")]));
        let mut t___16906: bool = cs__999.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___25 {}
        impl ClosureGroup___25 {
            fn fn__16898(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___25 {};
        let fn__16898 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16898())
        };
        test___35.assert(t___16906, fn__16898.clone());
        let mut t___16912: bool = Some(temper_core::ListedTrait::len( & cs__999.errors())) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___26 {}
        impl ClosureGroup___26 {
            fn fn__16897(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("no errors expected".to_string());
            }
        }
        let closure_group = ClosureGroup___26 {};
        let fn__16897 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16897())
        };
        test___35.assert(t___16912, fn__16897.clone());
        test___35.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredFailsWhenFieldMissing__2276() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___36 = temper_std::testing::Test::new();
        let params__1001: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___16877: TableDef = userTable__704();
        let mut t___16878: SafeIdentifier = csid__703("name");
        let cs__1002: Changeset = changeset(t___16877.clone(), params__1001.clone()).cast(std::sync::Arc::new(vec![t___16878.clone()])).validate_required(std::sync::Arc::new(vec![csid__703("name")]));
        let mut t___16884: bool = ! cs__1002.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___27 {}
        impl ClosureGroup___27 {
            fn fn__16875(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___27 {};
        let fn__16875 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16875())
        };
        test___36.assert(t___16884, fn__16875.clone());
        let mut t___16889: bool = Some(temper_core::ListedTrait::len( & cs__1002.errors())) == Some(1);
        #[derive(Clone)]
        struct ClosureGroup___28 {}
        impl ClosureGroup___28 {
            fn fn__16874(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have one error".to_string());
            }
        }
        let closure_group = ClosureGroup___28 {};
        let fn__16874 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16874())
        };
        test___36.assert(t___16889, fn__16874.clone());
        let mut t___16895: bool = Some(temper_core::ListedTrait::get( & cs__1002.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___29 {}
        impl ClosureGroup___29 {
            fn fn__16873(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error should name the field".to_string());
            }
        }
        let closure_group = ClosureGroup___29 {};
        let fn__16873 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16873())
        };
        test___36.assert(t___16895, fn__16873.clone());
        test___36.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthPassesWithinRange__2277() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___37 = temper_std::testing::Test::new();
        let params__1004: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___16865: TableDef = userTable__704();
        let mut t___16866: SafeIdentifier = csid__703("name");
        let cs__1005: Changeset = changeset(t___16865.clone(), params__1004.clone()).cast(std::sync::Arc::new(vec![t___16866.clone()])).validate_length(csid__703("name"), 2, 50);
        let mut t___16870: bool = cs__1005.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___30 {}
        impl ClosureGroup___30 {
            fn fn__16862(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___30 {};
        let fn__16862 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16862())
        };
        test___37.assert(t___16870, fn__16862.clone());
        test___37.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooShort__2278() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___38 = temper_std::testing::Test::new();
        let params__1007: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("A".to_string()))]);
        let mut t___16853: TableDef = userTable__704();
        let mut t___16854: SafeIdentifier = csid__703("name");
        let cs__1008: Changeset = changeset(t___16853.clone(), params__1007.clone()).cast(std::sync::Arc::new(vec![t___16854.clone()])).validate_length(csid__703("name"), 2, 50);
        let mut t___16860: bool = ! cs__1008.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___31 {}
        impl ClosureGroup___31 {
            fn fn__16850(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___31 {};
        let fn__16850 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16850())
        };
        test___38.assert(t___16860, fn__16850.clone());
        test___38.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooLong__2279() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___39 = temper_std::testing::Test::new();
        let params__1010: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()))]);
        let mut t___16841: TableDef = userTable__704();
        let mut t___16842: SafeIdentifier = csid__703("name");
        let cs__1011: Changeset = changeset(t___16841.clone(), params__1010.clone()).cast(std::sync::Arc::new(vec![t___16842.clone()])).validate_length(csid__703("name"), 2, 10);
        let mut t___16848: bool = ! cs__1011.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___32 {}
        impl ClosureGroup___32 {
            fn fn__16838(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___32 {};
        let fn__16838 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16838())
        };
        test___39.assert(t___16848, fn__16838.clone());
        test___39.soft_fail_to_hard()
    }
    #[test]
    fn validateIntPassesForValidInteger__2280() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___40 = temper_std::testing::Test::new();
        let params__1013: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("30".to_string()))]);
        let mut t___16830: TableDef = userTable__704();
        let mut t___16831: SafeIdentifier = csid__703("age");
        let cs__1014: Changeset = changeset(t___16830.clone(), params__1013.clone()).cast(std::sync::Arc::new(vec![t___16831.clone()])).validate_int(csid__703("age"));
        let mut t___16835: bool = cs__1014.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___33 {}
        impl ClosureGroup___33 {
            fn fn__16827(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___33 {};
        let fn__16827 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16827())
        };
        test___40.assert(t___16835, fn__16827.clone());
        test___40.soft_fail_to_hard()
    }
    #[test]
    fn validateIntFailsForNonInteger__2281() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___41 = temper_std::testing::Test::new();
        let params__1016: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___16818: TableDef = userTable__704();
        let mut t___16819: SafeIdentifier = csid__703("age");
        let cs__1017: Changeset = changeset(t___16818.clone(), params__1016.clone()).cast(std::sync::Arc::new(vec![t___16819.clone()])).validate_int(csid__703("age"));
        let mut t___16825: bool = ! cs__1017.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___34 {}
        impl ClosureGroup___34 {
            fn fn__16815(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___34 {};
        let fn__16815 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16815())
        };
        test___41.assert(t___16825, fn__16815.clone());
        test___41.soft_fail_to_hard()
    }
    #[test]
    fn validateFloatPassesForValidFloat__2282() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___42 = temper_std::testing::Test::new();
        let params__1019: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("9.5".to_string()))]);
        let mut t___16807: TableDef = userTable__704();
        let mut t___16808: SafeIdentifier = csid__703("score");
        let cs__1020: Changeset = changeset(t___16807.clone(), params__1019.clone()).cast(std::sync::Arc::new(vec![t___16808.clone()])).validate_float(csid__703("score"));
        let mut t___16812: bool = cs__1020.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___35 {}
        impl ClosureGroup___35 {
            fn fn__16804(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___35 {};
        let fn__16804 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16804())
        };
        test___42.assert(t___16812, fn__16804.clone());
        test___42.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_passesForValid64_bitInteger__2283() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___43 = temper_std::testing::Test::new();
        let params__1022: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("9999999999".to_string()))]);
        let mut t___16796: TableDef = userTable__704();
        let mut t___16797: SafeIdentifier = csid__703("age");
        let cs__1023: Changeset = changeset(t___16796.clone(), params__1022.clone()).cast(std::sync::Arc::new(vec![t___16797.clone()])).validate_int64(csid__703("age"));
        let mut t___16801: bool = cs__1023.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___36 {}
        impl ClosureGroup___36 {
            fn fn__16793(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___36 {};
        let fn__16793 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16793())
        };
        test___43.assert(t___16801, fn__16793.clone());
        test___43.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_failsForNonInteger__2284() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___44 = temper_std::testing::Test::new();
        let params__1025: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___16784: TableDef = userTable__704();
        let mut t___16785: SafeIdentifier = csid__703("age");
        let cs__1026: Changeset = changeset(t___16784.clone(), params__1025.clone()).cast(std::sync::Arc::new(vec![t___16785.clone()])).validate_int64(csid__703("age"));
        let mut t___16791: bool = ! cs__1026.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___37 {}
        impl ClosureGroup___37 {
            fn fn__16781(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___37 {};
        let fn__16781 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16781())
        };
        test___44.assert(t___16791, fn__16781.clone());
        test___44.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsTrue1_yesOn__2285() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___45 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___38 {
            test___45: temper_std::testing::Test
        }
        impl ClosureGroup___38 {
            fn fn__16778(& self, v__1028: impl temper_core::ToArcString) {
                let v__1028 = v__1028.to_arc_string();
                let params__1029: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__1028.clone())]);
                let mut t___16770: TableDef = userTable__704();
                let mut t___16771: SafeIdentifier = csid__703("active");
                let cs__1030: Changeset = changeset(t___16770.clone(), params__1029.clone()).cast(std::sync::Arc::new(vec![t___16771.clone()])).validate_bool(csid__703("active"));
                let mut t___16775: bool = cs__1030.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___39 {
                    v__1028: std::sync::Arc<String>
                }
                impl ClosureGroup___39 {
                    fn fn__16767(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__1028.clone()));
                    }
                }
                let closure_group = ClosureGroup___39 {
                    v__1028: v__1028.clone()
                };
                let fn__16767 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__16767())
                };
                self.test___45.assert(t___16775, fn__16767.clone());
            }
        }
        let closure_group = ClosureGroup___38 {
            test___45: test___45.clone()
        };
        let fn__16778 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__1028: std::sync::Arc<String> | closure_group.fn__16778(v__1028))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("true".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("yes".to_string()), std::sync::Arc::new("on".to_string())]), & ( * fn__16778.clone()));
        test___45.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsFalse0_noOff__2286() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___46 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___40 {
            test___46: temper_std::testing::Test
        }
        impl ClosureGroup___40 {
            fn fn__16764(& self, v__1032: impl temper_core::ToArcString) {
                let v__1032 = v__1032.to_arc_string();
                let params__1033: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__1032.clone())]);
                let mut t___16756: TableDef = userTable__704();
                let mut t___16757: SafeIdentifier = csid__703("active");
                let cs__1034: Changeset = changeset(t___16756.clone(), params__1033.clone()).cast(std::sync::Arc::new(vec![t___16757.clone()])).validate_bool(csid__703("active"));
                let mut t___16761: bool = cs__1034.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___41 {
                    v__1032: std::sync::Arc<String>
                }
                impl ClosureGroup___41 {
                    fn fn__16753(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__1032.clone()));
                    }
                }
                let closure_group = ClosureGroup___41 {
                    v__1032: v__1032.clone()
                };
                let fn__16753 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__16753())
                };
                self.test___46.assert(t___16761, fn__16753.clone());
            }
        }
        let closure_group = ClosureGroup___40 {
            test___46: test___46.clone()
        };
        let fn__16764 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__1032: std::sync::Arc<String> | closure_group.fn__16764(v__1032))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("false".to_string()), std::sync::Arc::new("0".to_string()), std::sync::Arc::new("no".to_string()), std::sync::Arc::new("off".to_string())]), & ( * fn__16764.clone()));
        test___46.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolRejectsAmbiguousValues__2287() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___47 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___42 {
            test___47: temper_std::testing::Test
        }
        impl ClosureGroup___42 {
            fn fn__16750(& self, v__1036: impl temper_core::ToArcString) {
                let v__1036 = v__1036.to_arc_string();
                let params__1037: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__1036.clone())]);
                let mut t___16741: TableDef = userTable__704();
                let mut t___16742: SafeIdentifier = csid__703("active");
                let cs__1038: Changeset = changeset(t___16741.clone(), params__1037.clone()).cast(std::sync::Arc::new(vec![t___16742.clone()])).validate_bool(csid__703("active"));
                let mut t___16748: bool = ! cs__1038.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___43 {
                    v__1036: std::sync::Arc<String>
                }
                impl ClosureGroup___43 {
                    fn fn__16738(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject ambiguous: {}", self.v__1036.clone()));
                    }
                }
                let closure_group = ClosureGroup___43 {
                    v__1036: v__1036.clone()
                };
                let fn__16738 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__16738())
                };
                self.test___47.assert(t___16748, fn__16738.clone());
            }
        }
        let closure_group = ClosureGroup___42 {
            test___47: test___47.clone()
        };
        let fn__16750 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__1036: std::sync::Arc<String> | closure_group.fn__16750(v__1036))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("TRUE".to_string()), std::sync::Arc::new("Yes".to_string()), std::sync::Arc::new("maybe".to_string()), std::sync::Arc::new("2".to_string()), std::sync::Arc::new("enabled".to_string())]), & ( * fn__16750.clone()));
        test___47.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEscapesBobbyTables__2288() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___48 = temper_std::testing::Test::new();
        let mut t___9222: SqlFragment;
        let params__1040: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Robert'); DROP TABLE users;--".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bobby@evil.com".to_string()))]);
        let mut t___16726: TableDef = userTable__704();
        let mut t___16727: SafeIdentifier = csid__703("name");
        let mut t___16728: SafeIdentifier = csid__703("email");
        let cs__1041: Changeset = changeset(t___16726.clone(), params__1040.clone()).cast(std::sync::Arc::new(vec![t___16727.clone(), t___16728.clone()])).validate_required(std::sync::Arc::new(vec![csid__703("name"), csid__703("email")]));
        let sqlFrag__1042: SqlFragment;
        'ok___17423: {
            'orelse___2740: {
                t___9222 = match cs__1041.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2740
                };
                sqlFrag__1042 = t___9222.clone();
                break 'ok___17423;
            }
            sqlFrag__1042 = panic!();
        }
        let s__1043: std::sync::Arc<String> = sqlFrag__1042.to_string();
        let mut t___16735: bool = temper_core::string::index_of( & s__1043, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___44 {
            s__1043: std::sync::Arc<String>
        }
        impl ClosureGroup___44 {
            fn fn__16722(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("single quote must be doubled: {}", self.s__1043.clone()));
            }
        }
        let closure_group = ClosureGroup___44 {
            s__1043: s__1043.clone()
        };
        let fn__16722 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16722())
        };
        test___48.assert(t___16735, fn__16722.clone());
        test___48.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForStringField__2289() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___49 = temper_std::testing::Test::new();
        let mut t___9201: SqlFragment;
        let params__1045: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___16706: TableDef = userTable__704();
        let mut t___16707: SafeIdentifier = csid__703("name");
        let mut t___16708: SafeIdentifier = csid__703("email");
        let cs__1046: Changeset = changeset(t___16706.clone(), params__1045.clone()).cast(std::sync::Arc::new(vec![t___16707.clone(), t___16708.clone()])).validate_required(std::sync::Arc::new(vec![csid__703("name"), csid__703("email")]));
        let sqlFrag__1047: SqlFragment;
        'ok___17424: {
            'orelse___2741: {
                t___9201 = match cs__1046.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2741
                };
                sqlFrag__1047 = t___9201.clone();
                break 'ok___17424;
            }
            sqlFrag__1047 = panic!();
        }
        let s__1048: std::sync::Arc<String> = sqlFrag__1047.to_string();
        let mut t___16715: bool = temper_core::string::index_of( & s__1048, "INSERT INTO users", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___45 {
            s__1048: std::sync::Arc<String>
        }
        impl ClosureGroup___45 {
            fn fn__16702(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has INSERT INTO: {}", self.s__1048.clone()));
            }
        }
        let closure_group = ClosureGroup___45 {
            s__1048: s__1048.clone()
        };
        let fn__16702 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16702())
        };
        test___49.assert(t___16715, fn__16702.clone());
        let mut t___16719: bool = temper_core::string::index_of( & s__1048, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___46 {
            s__1048: std::sync::Arc<String>
        }
        impl ClosureGroup___46 {
            fn fn__16701(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has quoted name: {}", self.s__1048.clone()));
            }
        }
        let closure_group = ClosureGroup___46 {
            s__1048: s__1048.clone()
        };
        let fn__16701 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16701())
        };
        test___49.assert(t___16719, fn__16701.clone());
        test___49.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForIntField__2290() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___50 = temper_std::testing::Test::new();
        let mut t___9184: SqlFragment;
        let params__1050: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("b@example.com".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___16688: TableDef = userTable__704();
        let mut t___16689: SafeIdentifier = csid__703("name");
        let mut t___16690: SafeIdentifier = csid__703("email");
        let mut t___16691: SafeIdentifier = csid__703("age");
        let cs__1051: Changeset = changeset(t___16688.clone(), params__1050.clone()).cast(std::sync::Arc::new(vec![t___16689.clone(), t___16690.clone(), t___16691.clone()])).validate_required(std::sync::Arc::new(vec![csid__703("name"), csid__703("email")]));
        let sqlFrag__1052: SqlFragment;
        'ok___17425: {
            'orelse___2742: {
                t___9184 = match cs__1051.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2742
                };
                sqlFrag__1052 = t___9184.clone();
                break 'ok___17425;
            }
            sqlFrag__1052 = panic!();
        }
        let s__1053: std::sync::Arc<String> = sqlFrag__1052.to_string();
        let mut t___16698: bool = temper_core::string::index_of( & s__1053, "25", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___47 {
            s__1053: std::sync::Arc<String>
        }
        impl ClosureGroup___47 {
            fn fn__16683(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("age rendered unquoted: {}", self.s__1053.clone()));
            }
        }
        let closure_group = ClosureGroup___47 {
            s__1053: s__1053.clone()
        };
        let fn__16683 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16683())
        };
        test___50.assert(t___16698, fn__16683.clone());
        test___50.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlBubblesOnInvalidChangeset__2291() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___51 = temper_std::testing::Test::new();
        let params__1055: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___16676: TableDef = userTable__704();
        let mut t___16677: SafeIdentifier = csid__703("name");
        let cs__1056: Changeset = changeset(t___16676.clone(), params__1055.clone()).cast(std::sync::Arc::new(vec![t___16677.clone()])).validate_required(std::sync::Arc::new(vec![csid__703("name")]));
        let didBubble__1057: bool;
        'ok___17426: {
            'orelse___2743: {
                match cs__1056.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2743
                };
                didBubble__1057 = false;
                break 'ok___17426;
            }
            didBubble__1057 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___48 {}
        impl ClosureGroup___48 {
            fn fn__16674(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___48 {};
        let fn__16674 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16674())
        };
        test___51.assert(didBubble__1057, fn__16674.clone());
        test___51.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__2292() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___52 = temper_std::testing::Test::new();
        let strictTable__1059: TableDef = TableDef::new(csid__703("posts"), [FieldDef::new(csid__703("title"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("body"), FieldType::new(StringField::new()), true, None, false)], None);
        let params__1060: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("body".to_string()), std::sync::Arc::new("hello".to_string()))]);
        let mut t___16667: SafeIdentifier = csid__703("body");
        let cs__1061: Changeset = changeset(strictTable__1059.clone(), params__1060.clone()).cast(std::sync::Arc::new(vec![t___16667.clone()]));
        let mut t___16669: bool = cs__1061.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___49 {}
        impl ClosureGroup___49 {
            fn fn__16656(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("changeset should appear valid (no explicit validation run)".to_string());
            }
        }
        let closure_group = ClosureGroup___49 {};
        let fn__16656 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16656())
        };
        test___52.assert(t___16669, fn__16656.clone());
        let didBubble__1062: bool;
        'ok___17427: {
            'orelse___2744: {
                match cs__1061.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2744
                };
                didBubble__1062 = false;
                break 'ok___17427;
            }
            didBubble__1062 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___50 {}
        impl ClosureGroup___50 {
            fn fn__16655(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("toInsertSql should enforce nullable regardless of isValid".to_string());
            }
        }
        let closure_group = ClosureGroup___50 {};
        let fn__16655 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16655())
        };
        test___52.assert(didBubble__1062, fn__16655.clone());
        test___52.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlProducesCorrectSql__2293() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___53 = temper_std::testing::Test::new();
        let mut t___9144: SqlFragment;
        let params__1064: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string()))]);
        let mut t___16646: TableDef = userTable__704();
        let mut t___16647: SafeIdentifier = csid__703("name");
        let cs__1065: Changeset = changeset(t___16646.clone(), params__1064.clone()).cast(std::sync::Arc::new(vec![t___16647.clone()])).validate_required(std::sync::Arc::new(vec![csid__703("name")]));
        let sqlFrag__1066: SqlFragment;
        'ok___17428: {
            'orelse___2745: {
                t___9144 = match cs__1065.to_update_sql(42) {
                    Ok(x) => x,
                    _ => break 'orelse___2745
                };
                sqlFrag__1066 = t___9144.clone();
                break 'ok___17428;
            }
            sqlFrag__1066 = panic!();
        }
        let s__1067: std::sync::Arc<String> = sqlFrag__1066.to_string();
        let mut t___16653: bool = Some(s__1067.as_str()) == Some("UPDATE users SET name = 'Bob' WHERE id = 42");
        #[derive(Clone)]
        struct ClosureGroup___51 {
            s__1067: std::sync::Arc<String>
        }
        impl ClosureGroup___51 {
            fn fn__16643(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__1067.clone()));
            }
        }
        let closure_group = ClosureGroup___51 {
            s__1067: s__1067.clone()
        };
        let fn__16643 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16643())
        };
        test___53.assert(t___16653, fn__16643.clone());
        test___53.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlBubblesOnInvalidChangeset__2294() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___54 = temper_std::testing::Test::new();
        let params__1069: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___16636: TableDef = userTable__704();
        let mut t___16637: SafeIdentifier = csid__703("name");
        let cs__1070: Changeset = changeset(t___16636.clone(), params__1069.clone()).cast(std::sync::Arc::new(vec![t___16637.clone()])).validate_required(std::sync::Arc::new(vec![csid__703("name")]));
        let didBubble__1071: bool;
        'ok___17429: {
            'orelse___2746: {
                match cs__1070.to_update_sql(1) {
                    Ok(x) => x,
                    _ => break 'orelse___2746
                };
                didBubble__1071 = false;
                break 'ok___17429;
            }
            didBubble__1071 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___52 {}
        impl ClosureGroup___52 {
            fn fn__16634(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___52 {};
        let fn__16634 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16634())
        };
        test___54.assert(didBubble__1071, fn__16634.clone());
        test___54.soft_fail_to_hard()
    }
    #[test]
    fn putChangeAddsANewField__2295() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___55 = temper_std::testing::Test::new();
        let params__1073: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___16620: TableDef = userTable__704();
        let mut t___16621: SafeIdentifier = csid__703("name");
        let cs__1074: Changeset = changeset(t___16620.clone(), params__1073.clone()).cast(std::sync::Arc::new(vec![t___16621.clone()])).put_change(csid__703("email"), std::sync::Arc::new("alice@example.com".to_string()));
        let mut t___16626: bool = temper_core::MappedTrait::has( & cs__1074.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___53 {}
        impl ClosureGroup___53 {
            fn fn__16617(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___53 {};
        let fn__16617 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16617())
        };
        test___55.assert(t___16626, fn__16617.clone());
        let mut t___16632: bool = Some(temper_core::MappedTrait::get_or( & cs__1074.changes(), std::sync::Arc::new("email".to_string()), std::sync::Arc::new("".to_string())).as_str()) == Some("alice@example.com");
        #[derive(Clone)]
        struct ClosureGroup___54 {}
        impl ClosureGroup___54 {
            fn fn__16616(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email value".to_string());
            }
        }
        let closure_group = ClosureGroup___54 {};
        let fn__16616 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16616())
        };
        test___55.assert(t___16632, fn__16616.clone());
        test___55.soft_fail_to_hard()
    }
    #[test]
    fn putChangeOverwritesExistingField__2296() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___56 = temper_std::testing::Test::new();
        let params__1076: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___16606: TableDef = userTable__704();
        let mut t___16607: SafeIdentifier = csid__703("name");
        let cs__1077: Changeset = changeset(t___16606.clone(), params__1076.clone()).cast(std::sync::Arc::new(vec![t___16607.clone()])).put_change(csid__703("name"), std::sync::Arc::new("Bob".to_string()));
        let mut t___16614: bool = Some(temper_core::MappedTrait::get_or( & cs__1077.changes(), std::sync::Arc::new("name".to_string()), std::sync::Arc::new("".to_string())).as_str()) == Some("Bob");
        #[derive(Clone)]
        struct ClosureGroup___55 {}
        impl ClosureGroup___55 {
            fn fn__16603(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should be overwritten".to_string());
            }
        }
        let closure_group = ClosureGroup___55 {};
        let fn__16603 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16603())
        };
        test___56.assert(t___16614, fn__16603.clone());
        test___56.soft_fail_to_hard()
    }
    #[test]
    fn putChangeValueAppearsInToInsertSql__2297() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___57 = temper_std::testing::Test::new();
        let mut t___9099: SqlFragment;
        let mut t___9100: SqlFragment;
        let params__1079: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___16592: TableDef = userTable__704();
        let mut t___16593: SafeIdentifier = csid__703("name");
        let mut t___16594: SafeIdentifier = csid__703("email");
        let cs__1080: Changeset = changeset(t___16592.clone(), params__1079.clone()).cast(std::sync::Arc::new(vec![t___16593.clone(), t___16594.clone()])).put_change(csid__703("name"), std::sync::Arc::new("Bob".to_string()));
        'ok___17430: {
            'orelse___2747: {
                t___9099 = match cs__1080.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2747
                };
                t___9100 = t___9099.clone();
                break 'ok___17430;
            }
            t___9100 = panic!();
        }
        let s__1081: std::sync::Arc<String> = t___9100.to_string();
        let mut t___16600: bool = temper_core::string::index_of( & s__1081, "'Bob'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___56 {
            s__1081: std::sync::Arc<String>
        }
        impl ClosureGroup___56 {
            fn fn__16588(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should use putChange value: {}", self.s__1081.clone()));
            }
        }
        let closure_group = ClosureGroup___56 {
            s__1081: s__1081.clone()
        };
        let fn__16588 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16588())
        };
        test___57.assert(t___16600, fn__16588.clone());
        test___57.soft_fail_to_hard()
    }
    #[test]
    fn getChangeReturnsValueForExistingField__2298() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___58 = temper_std::testing::Test::new();
        let mut t___9087: std::sync::Arc<String>;
        let params__1083: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___16581: TableDef = userTable__704();
        let mut t___16582: SafeIdentifier = csid__703("name");
        let cs__1084: Changeset = changeset(t___16581.clone(), params__1083.clone()).cast(std::sync::Arc::new(vec![t___16582.clone()]));
        let val__1085: std::sync::Arc<String>;
        'ok___17431: {
            'orelse___2748: {
                t___9087 = match cs__1084.get_change(csid__703("name")) {
                    Ok(x) => x,
                    _ => break 'orelse___2748
                };
                val__1085 = t___9087.clone();
                break 'ok___17431;
            }
            val__1085 = panic!();
        }
        let mut t___16586: bool = Some(val__1085.as_str()) == Some("Alice");
        #[derive(Clone)]
        struct ClosureGroup___57 {}
        impl ClosureGroup___57 {
            fn fn__16578(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should return Alice".to_string());
            }
        }
        let closure_group = ClosureGroup___57 {};
        let fn__16578 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16578())
        };
        test___58.assert(t___16586, fn__16578.clone());
        test___58.soft_fail_to_hard()
    }
    #[test]
    fn getChangeBubblesOnMissingField__2299() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___59 = temper_std::testing::Test::new();
        let params__1087: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___16572: TableDef = userTable__704();
        let mut t___16573: SafeIdentifier = csid__703("name");
        let cs__1088: Changeset = changeset(t___16572.clone(), params__1087.clone()).cast(std::sync::Arc::new(vec![t___16573.clone()]));
        let didBubble__1089: bool;
        'ok___17432: {
            'orelse___2749: {
                match cs__1088.get_change(csid__703("email")) {
                    Ok(x) => x,
                    _ => break 'orelse___2749
                };
                didBubble__1089 = false;
                break 'ok___17432;
            }
            didBubble__1089 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___58 {}
        impl ClosureGroup___58 {
            fn fn__16569(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should bubble for missing field".to_string());
            }
        }
        let closure_group = ClosureGroup___58 {};
        let fn__16569 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16569())
        };
        test___59.assert(didBubble__1089, fn__16569.clone());
        test___59.soft_fail_to_hard()
    }
    #[test]
    fn deleteChangeRemovesField__2300() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___60 = temper_std::testing::Test::new();
        let params__1091: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___16554: TableDef = userTable__704();
        let mut t___16555: SafeIdentifier = csid__703("name");
        let mut t___16556: SafeIdentifier = csid__703("email");
        let cs__1092: Changeset = changeset(t___16554.clone(), params__1091.clone()).cast(std::sync::Arc::new(vec![t___16555.clone(), t___16556.clone()])).delete_change(csid__703("email"));
        let mut t___16563: bool = ! temper_core::MappedTrait::has( & cs__1092.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___59 {}
        impl ClosureGroup___59 {
            fn fn__16550(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be removed".to_string());
            }
        }
        let closure_group = ClosureGroup___59 {};
        let fn__16550 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16550())
        };
        test___60.assert(t___16563, fn__16550.clone());
        let mut t___16566: bool = temper_core::MappedTrait::has( & cs__1092.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___60 {}
        impl ClosureGroup___60 {
            fn fn__16549(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should remain".to_string());
            }
        }
        let closure_group = ClosureGroup___60 {};
        let fn__16549 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16549())
        };
        test___60.assert(t___16566, fn__16549.clone());
        test___60.soft_fail_to_hard()
    }
    #[test]
    fn deleteChangeOnNonexistentFieldIsNoOp__2301() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___61 = temper_std::testing::Test::new();
        let params__1094: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___16537: TableDef = userTable__704();
        let mut t___16538: SafeIdentifier = csid__703("name");
        let cs__1095: Changeset = changeset(t___16537.clone(), params__1094.clone()).cast(std::sync::Arc::new(vec![t___16538.clone()])).delete_change(csid__703("email"));
        let mut t___16543: bool = temper_core::MappedTrait::has( & cs__1095.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___61 {}
        impl ClosureGroup___61 {
            fn fn__16534(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should still be present".to_string());
            }
        }
        let closure_group = ClosureGroup___61 {};
        let fn__16534 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16534())
        };
        test___61.assert(t___16543, fn__16534.clone());
        let mut t___16546: bool = cs__1095.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___62 {}
        impl ClosureGroup___62 {
            fn fn__16533(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___62 {};
        let fn__16533 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16533())
        };
        test___61.assert(t___16546, fn__16533.clone());
        test___61.soft_fail_to_hard()
    }
    #[test]
    fn validateInclusionPassesWhenValueInList__2302() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___62 = temper_std::testing::Test::new();
        let params__1097: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("admin".to_string()))]);
        let mut t___16525: TableDef = userTable__704();
        let mut t___16526: SafeIdentifier = csid__703("name");
        let cs__1098: Changeset = changeset(t___16525.clone(), params__1097.clone()).cast(std::sync::Arc::new(vec![t___16526.clone()])).validate_inclusion(csid__703("name"), std::sync::Arc::new(vec![std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("user".to_string()), std::sync::Arc::new("guest".to_string())]));
        let mut t___16530: bool = cs__1098.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___63 {}
        impl ClosureGroup___63 {
            fn fn__16522(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___63 {};
        let fn__16522 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16522())
        };
        test___62.assert(t___16530, fn__16522.clone());
        test___62.soft_fail_to_hard()
    }
    #[test]
    fn validateInclusionFailsWhenValueNotInList__2303() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___63 = temper_std::testing::Test::new();
        let params__1100: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("hacker".to_string()))]);
        let mut t___16507: TableDef = userTable__704();
        let mut t___16508: SafeIdentifier = csid__703("name");
        let cs__1101: Changeset = changeset(t___16507.clone(), params__1100.clone()).cast(std::sync::Arc::new(vec![t___16508.clone()])).validate_inclusion(csid__703("name"), std::sync::Arc::new(vec![std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("user".to_string()), std::sync::Arc::new("guest".to_string())]));
        let mut t___16514: bool = ! cs__1101.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___64 {}
        impl ClosureGroup___64 {
            fn fn__16504(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___64 {};
        let fn__16504 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16504())
        };
        test___63.assert(t___16514, fn__16504.clone());
        let mut t___16520: bool = Some(temper_core::ListedTrait::get( & cs__1101.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___65 {}
        impl ClosureGroup___65 {
            fn fn__16503(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error on name".to_string());
            }
        }
        let closure_group = ClosureGroup___65 {};
        let fn__16503 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16503())
        };
        test___63.assert(t___16520, fn__16503.clone());
        test___63.soft_fail_to_hard()
    }
    #[test]
    fn validateInclusionSkipsWhenFieldNotInChanges__2304() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___64 = temper_std::testing::Test::new();
        let params__1103: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___16495: TableDef = userTable__704();
        let mut t___16496: SafeIdentifier = csid__703("name");
        let cs__1104: Changeset = changeset(t___16495.clone(), params__1103.clone()).cast(std::sync::Arc::new(vec![t___16496.clone()])).validate_inclusion(csid__703("name"), std::sync::Arc::new(vec![std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("user".to_string())]));
        let mut t___16500: bool = cs__1104.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___66 {}
        impl ClosureGroup___66 {
            fn fn__16493(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___66 {};
        let fn__16493 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16493())
        };
        test___64.assert(t___16500, fn__16493.clone());
        test___64.soft_fail_to_hard()
    }
    #[test]
    fn validateExclusionPassesWhenValueNotInList__2305() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___65 = temper_std::testing::Test::new();
        let params__1106: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___16485: TableDef = userTable__704();
        let mut t___16486: SafeIdentifier = csid__703("name");
        let cs__1107: Changeset = changeset(t___16485.clone(), params__1106.clone()).cast(std::sync::Arc::new(vec![t___16486.clone()])).validate_exclusion(csid__703("name"), std::sync::Arc::new(vec![std::sync::Arc::new("root".to_string()), std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("superuser".to_string())]));
        let mut t___16490: bool = cs__1107.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___67 {}
        impl ClosureGroup___67 {
            fn fn__16482(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___67 {};
        let fn__16482 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16482())
        };
        test___65.assert(t___16490, fn__16482.clone());
        test___65.soft_fail_to_hard()
    }
    #[test]
    fn validateExclusionFailsWhenValueInList__2306() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___66 = temper_std::testing::Test::new();
        let params__1109: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("admin".to_string()))]);
        let mut t___16467: TableDef = userTable__704();
        let mut t___16468: SafeIdentifier = csid__703("name");
        let cs__1110: Changeset = changeset(t___16467.clone(), params__1109.clone()).cast(std::sync::Arc::new(vec![t___16468.clone()])).validate_exclusion(csid__703("name"), std::sync::Arc::new(vec![std::sync::Arc::new("root".to_string()), std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("superuser".to_string())]));
        let mut t___16474: bool = ! cs__1110.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___68 {}
        impl ClosureGroup___68 {
            fn fn__16464(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___68 {};
        let fn__16464 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16464())
        };
        test___66.assert(t___16474, fn__16464.clone());
        let mut t___16480: bool = Some(temper_core::ListedTrait::get( & cs__1110.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___69 {}
        impl ClosureGroup___69 {
            fn fn__16463(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error on name".to_string());
            }
        }
        let closure_group = ClosureGroup___69 {};
        let fn__16463 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16463())
        };
        test___66.assert(t___16480, fn__16463.clone());
        test___66.soft_fail_to_hard()
    }
    #[test]
    fn validateExclusionSkipsWhenFieldNotInChanges__2307() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___67 = temper_std::testing::Test::new();
        let params__1112: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___16455: TableDef = userTable__704();
        let mut t___16456: SafeIdentifier = csid__703("name");
        let cs__1113: Changeset = changeset(t___16455.clone(), params__1112.clone()).cast(std::sync::Arc::new(vec![t___16456.clone()])).validate_exclusion(csid__703("name"), std::sync::Arc::new(vec![std::sync::Arc::new("root".to_string()), std::sync::Arc::new("admin".to_string())]));
        let mut t___16460: bool = cs__1113.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___70 {}
        impl ClosureGroup___70 {
            fn fn__16453(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___70 {};
        let fn__16453 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16453())
        };
        test___67.assert(t___16460, fn__16453.clone());
        test___67.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberGreaterThanPasses__2308() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___68 = temper_std::testing::Test::new();
        let params__1115: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___16444: TableDef = userTable__704();
        let mut t___16445: SafeIdentifier = csid__703("age");
        let cs__1116: Changeset = changeset(t___16444.clone(), params__1115.clone()).cast(std::sync::Arc::new(vec![t___16445.clone()])).validate_number(csid__703("age"), NumberValidationOpts::new(Some(18.0f64), None, None, None, None));
        let mut t___16450: bool = cs__1116.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___71 {}
        impl ClosureGroup___71 {
            fn fn__16441(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("25 > 18 should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___71 {};
        let fn__16441 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16441())
        };
        test___68.assert(t___16450, fn__16441.clone());
        test___68.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberGreaterThanFails__2309() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___69 = temper_std::testing::Test::new();
        let params__1118: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("15".to_string()))]);
        let mut t___16431: TableDef = userTable__704();
        let mut t___16432: SafeIdentifier = csid__703("age");
        let cs__1119: Changeset = changeset(t___16431.clone(), params__1118.clone()).cast(std::sync::Arc::new(vec![t___16432.clone()])).validate_number(csid__703("age"), NumberValidationOpts::new(Some(18.0f64), None, None, None, None));
        let mut t___16439: bool = ! cs__1119.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___72 {}
        impl ClosureGroup___72 {
            fn fn__16428(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("15 > 18 should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___72 {};
        let fn__16428 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16428())
        };
        test___69.assert(t___16439, fn__16428.clone());
        test___69.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberLessThanPasses__2310() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___70 = temper_std::testing::Test::new();
        let params__1121: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("8.5".to_string()))]);
        let mut t___16419: TableDef = userTable__704();
        let mut t___16420: SafeIdentifier = csid__703("score");
        let cs__1122: Changeset = changeset(t___16419.clone(), params__1121.clone()).cast(std::sync::Arc::new(vec![t___16420.clone()])).validate_number(csid__703("score"), NumberValidationOpts::new(None, Some(10.0f64), None, None, None));
        let mut t___16425: bool = cs__1122.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___73 {}
        impl ClosureGroup___73 {
            fn fn__16416(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("8.5 < 10 should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___73 {};
        let fn__16416 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16416())
        };
        test___70.assert(t___16425, fn__16416.clone());
        test___70.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberLessThanFails__2311() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___71 = temper_std::testing::Test::new();
        let params__1124: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("12.0".to_string()))]);
        let mut t___16406: TableDef = userTable__704();
        let mut t___16407: SafeIdentifier = csid__703("score");
        let cs__1125: Changeset = changeset(t___16406.clone(), params__1124.clone()).cast(std::sync::Arc::new(vec![t___16407.clone()])).validate_number(csid__703("score"), NumberValidationOpts::new(None, Some(10.0f64), None, None, None));
        let mut t___16414: bool = ! cs__1125.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___74 {}
        impl ClosureGroup___74 {
            fn fn__16403(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("12 < 10 should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___74 {};
        let fn__16403 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16403())
        };
        test___71.assert(t___16414, fn__16403.clone());
        test___71.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberGreaterThanOrEqualBoundary__2312() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___72 = temper_std::testing::Test::new();
        let params__1127: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("18".to_string()))]);
        let mut t___16394: TableDef = userTable__704();
        let mut t___16395: SafeIdentifier = csid__703("age");
        let cs__1128: Changeset = changeset(t___16394.clone(), params__1127.clone()).cast(std::sync::Arc::new(vec![t___16395.clone()])).validate_number(csid__703("age"), NumberValidationOpts::new(None, None, Some(18.0f64), None, None));
        let mut t___16400: bool = cs__1128.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___75 {}
        impl ClosureGroup___75 {
            fn fn__16391(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("18 >= 18 should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___75 {};
        let fn__16391 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16391())
        };
        test___72.assert(t___16400, fn__16391.clone());
        test___72.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberCombinedOptions__2313() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___73 = temper_std::testing::Test::new();
        let params__1130: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("5.0".to_string()))]);
        let mut t___16382: TableDef = userTable__704();
        let mut t___16383: SafeIdentifier = csid__703("score");
        let cs__1131: Changeset = changeset(t___16382.clone(), params__1130.clone()).cast(std::sync::Arc::new(vec![t___16383.clone()])).validate_number(csid__703("score"), NumberValidationOpts::new(Some(0.0f64), Some(10.0f64), None, None, None));
        let mut t___16388: bool = cs__1131.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___76 {}
        impl ClosureGroup___76 {
            fn fn__16379(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("5 > 0 and < 10 should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___76 {};
        let fn__16379 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16379())
        };
        test___73.assert(t___16388, fn__16379.clone());
        test___73.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberNonNumericValue__2314() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___74 = temper_std::testing::Test::new();
        let params__1133: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("abc".to_string()))]);
        let mut t___16363: TableDef = userTable__704();
        let mut t___16364: SafeIdentifier = csid__703("age");
        let cs__1134: Changeset = changeset(t___16363.clone(), params__1133.clone()).cast(std::sync::Arc::new(vec![t___16364.clone()])).validate_number(csid__703("age"), NumberValidationOpts::new(Some(0.0f64), None, None, None, None));
        let mut t___16371: bool = ! cs__1134.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___77 {}
        impl ClosureGroup___77 {
            fn fn__16360(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("non-numeric should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___77 {};
        let fn__16360 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16360())
        };
        test___74.assert(t___16371, fn__16360.clone());
        let mut t___16377: bool = Some(temper_core::ListedTrait::get( & cs__1134.errors(), 0).message().as_str()) == Some("must be a number");
        #[derive(Clone)]
        struct ClosureGroup___78 {}
        impl ClosureGroup___78 {
            fn fn__16359(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("correct error message".to_string());
            }
        }
        let closure_group = ClosureGroup___78 {};
        let fn__16359 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16359())
        };
        test___74.assert(t___16377, fn__16359.clone());
        test___74.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberSkipsWhenFieldNotInChanges__2315() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___75 = temper_std::testing::Test::new();
        let params__1136: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___16350: TableDef = userTable__704();
        let mut t___16351: SafeIdentifier = csid__703("age");
        let cs__1137: Changeset = changeset(t___16350.clone(), params__1136.clone()).cast(std::sync::Arc::new(vec![t___16351.clone()])).validate_number(csid__703("age"), NumberValidationOpts::new(Some(0.0f64), None, None, None, None));
        let mut t___16356: bool = cs__1137.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___79 {}
        impl ClosureGroup___79 {
            fn fn__16348(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___79 {};
        let fn__16348 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16348())
        };
        test___75.assert(t___16356, fn__16348.clone());
        test___75.soft_fail_to_hard()
    }
    #[test]
    fn validateAcceptancePassesForTrueValues__2316() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___76 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___80 {
            test___76: temper_std::testing::Test
        }
        impl ClosureGroup___80 {
            fn fn__16345(& self, v__1139: impl temper_core::ToArcString) {
                let v__1139 = v__1139.to_arc_string();
                let params__1140: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__1139.clone())]);
                let mut t___16337: TableDef = userTable__704();
                let mut t___16338: SafeIdentifier = csid__703("active");
                let cs__1141: Changeset = changeset(t___16337.clone(), params__1140.clone()).cast(std::sync::Arc::new(vec![t___16338.clone()])).validate_acceptance(csid__703("active"));
                let mut t___16342: bool = cs__1141.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___81 {
                    v__1139: std::sync::Arc<String>
                }
                impl ClosureGroup___81 {
                    fn fn__16334(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__1139.clone()));
                    }
                }
                let closure_group = ClosureGroup___81 {
                    v__1139: v__1139.clone()
                };
                let fn__16334 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__16334())
                };
                self.test___76.assert(t___16342, fn__16334.clone());
            }
        }
        let closure_group = ClosureGroup___80 {
            test___76: test___76.clone()
        };
        let fn__16345 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__1139: std::sync::Arc<String> | closure_group.fn__16345(v__1139))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("true".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("yes".to_string()), std::sync::Arc::new("on".to_string())]), & ( * fn__16345.clone()));
        test___76.soft_fail_to_hard()
    }
    #[test]
    fn validateAcceptanceFailsForNonTrueValues__2317() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___77 = temper_std::testing::Test::new();
        let params__1143: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), std::sync::Arc::new("false".to_string()))]);
        let mut t___16319: TableDef = userTable__704();
        let mut t___16320: SafeIdentifier = csid__703("active");
        let cs__1144: Changeset = changeset(t___16319.clone(), params__1143.clone()).cast(std::sync::Arc::new(vec![t___16320.clone()])).validate_acceptance(csid__703("active"));
        let mut t___16326: bool = ! cs__1144.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___82 {}
        impl ClosureGroup___82 {
            fn fn__16316(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("false should not be accepted".to_string());
            }
        }
        let closure_group = ClosureGroup___82 {};
        let fn__16316 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16316())
        };
        test___77.assert(t___16326, fn__16316.clone());
        let mut t___16332: bool = Some(temper_core::ListedTrait::get( & cs__1144.errors(), 0).message().as_str()) == Some("must be accepted");
        #[derive(Clone)]
        struct ClosureGroup___83 {}
        impl ClosureGroup___83 {
            fn fn__16315(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("correct message".to_string());
            }
        }
        let closure_group = ClosureGroup___83 {};
        let fn__16315 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16315())
        };
        test___77.assert(t___16332, fn__16315.clone());
        test___77.soft_fail_to_hard()
    }
    #[test]
    fn validateConfirmationPassesWhenFieldsMatch__2318() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___78 = temper_std::testing::Test::new();
        let tbl__1146: TableDef = TableDef::new(csid__703("users"), [FieldDef::new(csid__703("password"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("password_confirmation"), FieldType::new(StringField::new()), true, None, false)], None);
        let params__1147: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("password".to_string()), std::sync::Arc::new("secret123".to_string())), (std::sync::Arc::new("password_confirmation".to_string()), std::sync::Arc::new("secret123".to_string()))]);
        let mut t___16306: SafeIdentifier = csid__703("password");
        let mut t___16307: SafeIdentifier = csid__703("password_confirmation");
        let cs__1148: Changeset = changeset(tbl__1146.clone(), params__1147.clone()).cast(std::sync::Arc::new(vec![t___16306.clone(), t___16307.clone()])).validate_confirmation(csid__703("password"), csid__703("password_confirmation"));
        let mut t___16312: bool = cs__1148.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___84 {}
        impl ClosureGroup___84 {
            fn fn__16294(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("matching fields should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___84 {};
        let fn__16294 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16294())
        };
        test___78.assert(t___16312, fn__16294.clone());
        test___78.soft_fail_to_hard()
    }
    #[test]
    fn validateConfirmationFailsWhenFieldsDiffer__2319() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___79 = temper_std::testing::Test::new();
        let tbl__1150: TableDef = TableDef::new(csid__703("users"), [FieldDef::new(csid__703("password"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("password_confirmation"), FieldType::new(StringField::new()), true, None, false)], None);
        let params__1151: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("password".to_string()), std::sync::Arc::new("secret123".to_string())), (std::sync::Arc::new("password_confirmation".to_string()), std::sync::Arc::new("wrong456".to_string()))]);
        let mut t___16278: SafeIdentifier = csid__703("password");
        let mut t___16279: SafeIdentifier = csid__703("password_confirmation");
        let cs__1152: Changeset = changeset(tbl__1150.clone(), params__1151.clone()).cast(std::sync::Arc::new(vec![t___16278.clone(), t___16279.clone()])).validate_confirmation(csid__703("password"), csid__703("password_confirmation"));
        let mut t___16286: bool = ! cs__1152.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___85 {}
        impl ClosureGroup___85 {
            fn fn__16266(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mismatched fields should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___85 {};
        let fn__16266 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16266())
        };
        test___79.assert(t___16286, fn__16266.clone());
        let mut t___16292: bool = Some(temper_core::ListedTrait::get( & cs__1152.errors(), 0).field().as_str()) == Some("password_confirmation");
        #[derive(Clone)]
        struct ClosureGroup___86 {}
        impl ClosureGroup___86 {
            fn fn__16265(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error on confirmation field".to_string());
            }
        }
        let closure_group = ClosureGroup___86 {};
        let fn__16265 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16265())
        };
        test___79.assert(t___16292, fn__16265.clone());
        test___79.soft_fail_to_hard()
    }
    #[test]
    fn validateConfirmationFailsWhenConfirmationMissing__2320() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___80 = temper_std::testing::Test::new();
        let tbl__1154: TableDef = TableDef::new(csid__703("users"), [FieldDef::new(csid__703("password"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("password_confirmation"), FieldType::new(StringField::new()), true, None, false)], None);
        let params__1155: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("password".to_string()), std::sync::Arc::new("secret123".to_string()))]);
        let mut t___16256: SafeIdentifier = csid__703("password");
        let cs__1156: Changeset = changeset(tbl__1154.clone(), params__1155.clone()).cast(std::sync::Arc::new(vec![t___16256.clone()])).validate_confirmation(csid__703("password"), csid__703("password_confirmation"));
        let mut t___16263: bool = ! cs__1156.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___87 {}
        impl ClosureGroup___87 {
            fn fn__16245(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("missing confirmation should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___87 {};
        let fn__16245 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16245())
        };
        test___80.assert(t___16263, fn__16245.clone());
        test___80.soft_fail_to_hard()
    }
    #[test]
    fn validateContainsPassesWhenSubstringFound__2321() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___81 = temper_std::testing::Test::new();
        let params__1158: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___16237: TableDef = userTable__704();
        let mut t___16238: SafeIdentifier = csid__703("email");
        let cs__1159: Changeset = changeset(t___16237.clone(), params__1158.clone()).cast(std::sync::Arc::new(vec![t___16238.clone()])).validate_contains(csid__703("email"), std::sync::Arc::new("@".to_string()));
        let mut t___16242: bool = cs__1159.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___88 {}
        impl ClosureGroup___88 {
            fn fn__16234(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should pass when @ present".to_string());
            }
        }
        let closure_group = ClosureGroup___88 {};
        let fn__16234 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16234())
        };
        test___81.assert(t___16242, fn__16234.clone());
        test___81.soft_fail_to_hard()
    }
    #[test]
    fn validateContainsFailsWhenSubstringNotFound__2322() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___82 = temper_std::testing::Test::new();
        let params__1161: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice-example.com".to_string()))]);
        let mut t___16225: TableDef = userTable__704();
        let mut t___16226: SafeIdentifier = csid__703("email");
        let cs__1162: Changeset = changeset(t___16225.clone(), params__1161.clone()).cast(std::sync::Arc::new(vec![t___16226.clone()])).validate_contains(csid__703("email"), std::sync::Arc::new("@".to_string()));
        let mut t___16232: bool = ! cs__1162.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___89 {}
        impl ClosureGroup___89 {
            fn fn__16222(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should fail when @ absent".to_string());
            }
        }
        let closure_group = ClosureGroup___89 {};
        let fn__16222 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16222())
        };
        test___82.assert(t___16232, fn__16222.clone());
        test___82.soft_fail_to_hard()
    }
    #[test]
    fn validateContainsSkipsWhenFieldNotInChanges__2323() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___83 = temper_std::testing::Test::new();
        let params__1164: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___16214: TableDef = userTable__704();
        let mut t___16215: SafeIdentifier = csid__703("email");
        let cs__1165: Changeset = changeset(t___16214.clone(), params__1164.clone()).cast(std::sync::Arc::new(vec![t___16215.clone()])).validate_contains(csid__703("email"), std::sync::Arc::new("@".to_string()));
        let mut t___16219: bool = cs__1165.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___90 {}
        impl ClosureGroup___90 {
            fn fn__16212(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___90 {};
        let fn__16212 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16212())
        };
        test___83.assert(t___16219, fn__16212.clone());
        test___83.soft_fail_to_hard()
    }
    #[test]
    fn validateStartsWithPasses__2324() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___84 = temper_std::testing::Test::new();
        let params__1167: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Dr. Smith".to_string()))]);
        let mut t___16204: TableDef = userTable__704();
        let mut t___16205: SafeIdentifier = csid__703("name");
        let cs__1168: Changeset = changeset(t___16204.clone(), params__1167.clone()).cast(std::sync::Arc::new(vec![t___16205.clone()])).validate_starts_with(csid__703("name"), std::sync::Arc::new("Dr.".to_string()));
        let mut t___16209: bool = cs__1168.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___91 {}
        impl ClosureGroup___91 {
            fn fn__16201(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should pass for Dr. prefix".to_string());
            }
        }
        let closure_group = ClosureGroup___91 {};
        let fn__16201 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16201())
        };
        test___84.assert(t___16209, fn__16201.clone());
        test___84.soft_fail_to_hard()
    }
    #[test]
    fn validateStartsWithFails__2325() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___85 = temper_std::testing::Test::new();
        let params__1170: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Mr. Smith".to_string()))]);
        let mut t___16192: TableDef = userTable__704();
        let mut t___16193: SafeIdentifier = csid__703("name");
        let cs__1171: Changeset = changeset(t___16192.clone(), params__1170.clone()).cast(std::sync::Arc::new(vec![t___16193.clone()])).validate_starts_with(csid__703("name"), std::sync::Arc::new("Dr.".to_string()));
        let mut t___16199: bool = ! cs__1171.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___92 {}
        impl ClosureGroup___92 {
            fn fn__16189(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should fail for Mr. prefix".to_string());
            }
        }
        let closure_group = ClosureGroup___92 {};
        let fn__16189 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16189())
        };
        test___85.assert(t___16199, fn__16189.clone());
        test___85.soft_fail_to_hard()
    }
    #[test]
    fn validateEndsWithPasses__2326() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___86 = temper_std::testing::Test::new();
        let params__1173: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___16181: TableDef = userTable__704();
        let mut t___16182: SafeIdentifier = csid__703("email");
        let cs__1174: Changeset = changeset(t___16181.clone(), params__1173.clone()).cast(std::sync::Arc::new(vec![t___16182.clone()])).validate_ends_with(csid__703("email"), std::sync::Arc::new(".com".to_string()));
        let mut t___16186: bool = cs__1174.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___93 {}
        impl ClosureGroup___93 {
            fn fn__16178(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should pass for .com suffix".to_string());
            }
        }
        let closure_group = ClosureGroup___93 {};
        let fn__16178 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16178())
        };
        test___86.assert(t___16186, fn__16178.clone());
        test___86.soft_fail_to_hard()
    }
    #[test]
    fn validateEndsWithFails__2327() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___87 = temper_std::testing::Test::new();
        let params__1176: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.org".to_string()))]);
        let mut t___16169: TableDef = userTable__704();
        let mut t___16170: SafeIdentifier = csid__703("email");
        let cs__1177: Changeset = changeset(t___16169.clone(), params__1176.clone()).cast(std::sync::Arc::new(vec![t___16170.clone()])).validate_ends_with(csid__703("email"), std::sync::Arc::new(".com".to_string()));
        let mut t___16176: bool = ! cs__1177.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___94 {}
        impl ClosureGroup___94 {
            fn fn__16166(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should fail for .org when expecting .com".to_string());
            }
        }
        let closure_group = ClosureGroup___94 {};
        let fn__16166 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16166())
        };
        test___87.assert(t___16176, fn__16166.clone());
        test___87.soft_fail_to_hard()
    }
    #[test]
    fn validateEndsWithHandlesRepeatedSuffixCorrectly__2328() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___88 = temper_std::testing::Test::new();
        let params__1179: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("abcabc".to_string()))]);
        let mut t___16158: TableDef = userTable__704();
        let mut t___16159: SafeIdentifier = csid__703("name");
        let cs__1180: Changeset = changeset(t___16158.clone(), params__1179.clone()).cast(std::sync::Arc::new(vec![t___16159.clone()])).validate_ends_with(csid__703("name"), std::sync::Arc::new("abc".to_string()));
        let mut t___16163: bool = cs__1180.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___95 {}
        impl ClosureGroup___95 {
            fn fn__16155(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("abcabc should end with abc".to_string());
            }
        }
        let closure_group = ClosureGroup___95 {};
        let fn__16155 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16155())
        };
        test___88.assert(t___16163, fn__16155.clone());
        test___88.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlUsesDefaultValueWhenFieldNotInChanges__2329() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___89 = temper_std::testing::Test::new();
        let mut t___8735: SqlFragment;
        let mut t___8736: SqlFragment;
        let tbl__1182: TableDef = TableDef::new(csid__703("posts"), [FieldDef::new(csid__703("title"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("status"), FieldType::new(StringField::new()), false, Some(SqlPart::new(SqlDefault::new())), false)], None);
        let params__1183: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("title".to_string()), std::sync::Arc::new("Hello".to_string()))]);
        let mut t___16139: SafeIdentifier = csid__703("title");
        let cs__1184: Changeset = changeset(tbl__1182.clone(), params__1183.clone()).cast(std::sync::Arc::new(vec![t___16139.clone()]));
        'ok___17433: {
            'orelse___2750: {
                t___8735 = match cs__1184.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2750
                };
                t___8736 = t___8735.clone();
                break 'ok___17433;
            }
            t___8736 = panic!();
        }
        let s__1185: std::sync::Arc<String> = t___8736.to_string();
        let mut t___16143: bool = temper_core::string::index_of( & s__1185, "INSERT INTO posts", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___96 {
            s__1185: std::sync::Arc<String>
        }
        impl ClosureGroup___96 {
            fn fn__16127(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has INSERT INTO: {}", self.s__1185.clone()));
            }
        }
        let closure_group = ClosureGroup___96 {
            s__1185: s__1185.clone()
        };
        let fn__16127 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16127())
        };
        test___89.assert(t___16143, fn__16127.clone());
        let mut t___16147: bool = temper_core::string::index_of( & s__1185, "'Hello'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___97 {
            s__1185: std::sync::Arc<String>
        }
        impl ClosureGroup___97 {
            fn fn__16126(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has title value: {}", self.s__1185.clone()));
            }
        }
        let closure_group = ClosureGroup___97 {
            s__1185: s__1185.clone()
        };
        let fn__16126 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16126())
        };
        test___89.assert(t___16147, fn__16126.clone());
        let mut t___16151: bool = temper_core::string::index_of( & s__1185, "DEFAULT", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___98 {
            s__1185: std::sync::Arc<String>
        }
        impl ClosureGroup___98 {
            fn fn__16125(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("status should use DEFAULT: {}", self.s__1185.clone()));
            }
        }
        let closure_group = ClosureGroup___98 {
            s__1185: s__1185.clone()
        };
        let fn__16125 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16125())
        };
        test___89.assert(t___16151, fn__16125.clone());
        test___89.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlChangeOverridesDefaultValue__2330() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___90 = temper_std::testing::Test::new();
        let mut t___8715: SqlFragment;
        let mut t___8716: SqlFragment;
        let tbl__1187: TableDef = TableDef::new(csid__703("posts"), [FieldDef::new(csid__703("title"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("status"), FieldType::new(StringField::new()), false, Some(SqlPart::new(SqlDefault::new())), false)], None);
        let params__1188: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("title".to_string()), std::sync::Arc::new("Hello".to_string())), (std::sync::Arc::new("status".to_string()), std::sync::Arc::new("published".to_string()))]);
        let mut t___16117: SafeIdentifier = csid__703("title");
        let mut t___16118: SafeIdentifier = csid__703("status");
        let cs__1189: Changeset = changeset(tbl__1187.clone(), params__1188.clone()).cast(std::sync::Arc::new(vec![t___16117.clone(), t___16118.clone()]));
        'ok___17434: {
            'orelse___2751: {
                t___8715 = match cs__1189.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2751
                };
                t___8716 = t___8715.clone();
                break 'ok___17434;
            }
            t___8716 = panic!();
        }
        let s__1190: std::sync::Arc<String> = t___8716.to_string();
        let mut t___16122: bool = temper_core::string::index_of( & s__1190, "'published'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___99 {
            s__1190: std::sync::Arc<String>
        }
        impl ClosureGroup___99 {
            fn fn__16104(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should use provided value: {}", self.s__1190.clone()));
            }
        }
        let closure_group = ClosureGroup___99 {
            s__1190: s__1190.clone()
        };
        let fn__16104 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16104())
        };
        test___90.assert(t___16122, fn__16104.clone());
        test___90.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlWithTimestampsUsesDefault__2331() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___91 = temper_std::testing::Test::new();
        let mut t___8662: temper_core::List<FieldDef>;
        let mut t___8677: SqlFragment;
        let mut t___8678: SqlFragment;
        let ts__1192: temper_core::List<FieldDef>;
        'ok___17435: {
            'orelse___2752: {
                t___8662 = match timestamps() {
                    Ok(x) => x,
                    _ => break 'orelse___2752
                };
                ts__1192 = t___8662.clone();
                break 'ok___17435;
            }
            ts__1192 = panic!();
        }
        let fields__1193: temper_core::ListBuilder<FieldDef> = temper_core::listed::new_builder();
        temper_core::listed::add( & fields__1193, FieldDef::new(csid__703("title"), FieldType::new(StringField::new()), false, None, false), None);
        #[derive(Clone)]
        struct ClosureGroup___100 {
            fields__1193: temper_core::ListBuilder<FieldDef>
        }
        impl ClosureGroup___100 {
            fn fn__16070(& self, t__1194: FieldDef) {
                temper_core::listed::add( & self.fields__1193, t__1194.clone(), None);
            }
        }
        let closure_group = ClosureGroup___100 {
            fields__1193: fields__1193.clone()
        };
        let fn__16070 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | t__1194: FieldDef | closure_group.fn__16070(t__1194))
        };
        temper_core::listed::list_for_each( & ts__1192, & ( * fn__16070.clone()));
        let tbl__1195: TableDef = TableDef::new(csid__703("articles"), temper_core::ListedTrait::to_list( & fields__1193), None);
        let params__1196: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("title".to_string()), std::sync::Arc::new("News".to_string()))]);
        let mut t___16083: SafeIdentifier = csid__703("title");
        let cs__1197: Changeset = changeset(tbl__1195.clone(), params__1196.clone()).cast(std::sync::Arc::new(vec![t___16083.clone()]));
        'ok___17436: {
            'orelse___2753: {
                t___8677 = match cs__1197.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2753
                };
                t___8678 = t___8677.clone();
                break 'ok___17436;
            }
            t___8678 = panic!();
        }
        let s__1198: std::sync::Arc<String> = t___8678.to_string();
        let mut t___16087: bool = temper_core::string::index_of( & s__1198, "inserted_at", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___101 {
            s__1198: std::sync::Arc<String>
        }
        impl ClosureGroup___101 {
            fn fn__16069(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should include inserted_at: {}", self.s__1198.clone()));
            }
        }
        let closure_group = ClosureGroup___101 {
            s__1198: s__1198.clone()
        };
        let fn__16069 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16069())
        };
        test___91.assert(t___16087, fn__16069.clone());
        let mut t___16091: bool = temper_core::string::index_of( & s__1198, "updated_at", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___102 {
            s__1198: std::sync::Arc<String>
        }
        impl ClosureGroup___102 {
            fn fn__16068(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should include updated_at: {}", self.s__1198.clone()));
            }
        }
        let closure_group = ClosureGroup___102 {
            s__1198: s__1198.clone()
        };
        let fn__16068 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16068())
        };
        test___91.assert(t___16091, fn__16068.clone());
        let mut t___16095: bool = temper_core::string::index_of( & s__1198, "DEFAULT", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___103 {
            s__1198: std::sync::Arc<String>
        }
        impl ClosureGroup___103 {
            fn fn__16067(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("timestamps should use DEFAULT: {}", self.s__1198.clone()));
            }
        }
        let closure_group = ClosureGroup___103 {
            s__1198: s__1198.clone()
        };
        let fn__16067 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16067())
        };
        test___91.assert(t___16095, fn__16067.clone());
        test___91.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlSkipsVirtualFields__2332() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___92 = temper_std::testing::Test::new();
        let mut t___8651: SqlFragment;
        let mut t___8652: SqlFragment;
        let tbl__1200: TableDef = TableDef::new(csid__703("users"), [FieldDef::new(csid__703("name"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("full_name"), FieldType::new(StringField::new()), true, None, true)], None);
        let params__1201: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("full_name".to_string()), std::sync::Arc::new("Alice Smith".to_string()))]);
        let mut t___16053: SafeIdentifier = csid__703("name");
        let mut t___16054: SafeIdentifier = csid__703("full_name");
        let cs__1202: Changeset = changeset(tbl__1200.clone(), params__1201.clone()).cast(std::sync::Arc::new(vec![t___16053.clone(), t___16054.clone()]));
        'ok___17437: {
            'orelse___2754: {
                t___8651 = match cs__1202.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2754
                };
                t___8652 = t___8651.clone();
                break 'ok___17437;
            }
            t___8652 = panic!();
        }
        let s__1203: std::sync::Arc<String> = t___8652.to_string();
        let mut t___16058: bool = temper_core::string::index_of( & s__1203, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___104 {
            s__1203: std::sync::Arc<String>
        }
        impl ClosureGroup___104 {
            fn fn__16041(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("name should be included: {}", self.s__1203.clone()));
            }
        }
        let closure_group = ClosureGroup___104 {
            s__1203: s__1203.clone()
        };
        let fn__16041 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16041())
        };
        test___92.assert(t___16058, fn__16041.clone());
        let mut t___16064: bool = ! temper_core::string::index_of( & s__1203, "full_name", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___105 {
            s__1203: std::sync::Arc<String>
        }
        impl ClosureGroup___105 {
            fn fn__16040(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("virtual field should be excluded: {}", self.s__1203.clone()));
            }
        }
        let closure_group = ClosureGroup___105 {
            s__1203: s__1203.clone()
        };
        let fn__16040 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16040())
        };
        test___92.assert(t___16064, fn__16040.clone());
        test___92.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlAllowsMissingNonNullableVirtualField__2333() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___93 = temper_std::testing::Test::new();
        let mut t___8630: SqlFragment;
        let mut t___8631: SqlFragment;
        let tbl__1205: TableDef = TableDef::new(csid__703("users"), [FieldDef::new(csid__703("name"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("computed"), FieldType::new(StringField::new()), false, None, true)], None);
        let params__1206: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___16033: SafeIdentifier = csid__703("name");
        let cs__1207: Changeset = changeset(tbl__1205.clone(), params__1206.clone()).cast(std::sync::Arc::new(vec![t___16033.clone()]));
        'ok___17438: {
            'orelse___2755: {
                t___8630 = match cs__1207.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2755
                };
                t___8631 = t___8630.clone();
                break 'ok___17438;
            }
            t___8631 = panic!();
        }
        let s__1208: std::sync::Arc<String> = t___8631.to_string();
        let mut t___16037: bool = temper_core::string::index_of( & s__1208, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___106 {
            s__1208: std::sync::Arc<String>
        }
        impl ClosureGroup___106 {
            fn fn__16022(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should succeed: {}", self.s__1208.clone()));
            }
        }
        let closure_group = ClosureGroup___106 {
            s__1208: s__1208.clone()
        };
        let fn__16022 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__16022())
        };
        test___93.assert(t___16037, fn__16022.clone());
        test___93.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlSkipsVirtualFields__2334() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___94 = temper_std::testing::Test::new();
        let mut t___8607: SqlFragment;
        let mut t___8608: SqlFragment;
        let tbl__1210: TableDef = TableDef::new(csid__703("users"), [FieldDef::new(csid__703("name"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("display"), FieldType::new(StringField::new()), true, None, true)], None);
        let params__1211: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("display".to_string()), std::sync::Arc::new("Bobby".to_string()))]);
        let mut t___16009: SafeIdentifier = csid__703("name");
        let mut t___16010: SafeIdentifier = csid__703("display");
        let cs__1212: Changeset = changeset(tbl__1210.clone(), params__1211.clone()).cast(std::sync::Arc::new(vec![t___16009.clone(), t___16010.clone()]));
        'ok___17439: {
            'orelse___2756: {
                t___8607 = match cs__1212.to_update_sql(1) {
                    Ok(x) => x,
                    _ => break 'orelse___2756
                };
                t___8608 = t___8607.clone();
                break 'ok___17439;
            }
            t___8608 = panic!();
        }
        let s__1213: std::sync::Arc<String> = t___8608.to_string();
        let mut t___16014: bool = temper_core::string::index_of( & s__1213, "name = 'Bob'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___107 {
            s__1213: std::sync::Arc<String>
        }
        impl ClosureGroup___107 {
            fn fn__15997(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("name should be in SET: {}", self.s__1213.clone()));
            }
        }
        let closure_group = ClosureGroup___107 {
            s__1213: s__1213.clone()
        };
        let fn__15997 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15997())
        };
        test___94.assert(t___16014, fn__15997.clone());
        let mut t___16020: bool = ! temper_core::string::index_of( & s__1213, "display", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___108 {
            s__1213: std::sync::Arc<String>
        }
        impl ClosureGroup___108 {
            fn fn__15996(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("virtual field excluded from UPDATE: {}", self.s__1213.clone()));
            }
        }
        let closure_group = ClosureGroup___108 {
            s__1213: s__1213.clone()
        };
        let fn__15996 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15996())
        };
        test___94.assert(t___16020, fn__15996.clone());
        test___94.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlUsesCustomPrimaryKey__2335() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___95 = temper_std::testing::Test::new();
        let mut t___8589: SqlFragment;
        let mut t___8590: SqlFragment;
        let tbl__1215: TableDef = TableDef::new(csid__703("posts"), [FieldDef::new(csid__703("title"), FieldType::new(StringField::new()), false, None, false)], Some(csid__703("post_id")));
        let params__1216: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("title".to_string()), std::sync::Arc::new("Updated".to_string()))]);
        let mut t___15990: SafeIdentifier = csid__703("title");
        let cs__1217: Changeset = changeset(tbl__1215.clone(), params__1216.clone()).cast(std::sync::Arc::new(vec![t___15990.clone()]));
        'ok___17440: {
            'orelse___2757: {
                t___8589 = match cs__1217.to_update_sql(99) {
                    Ok(x) => x,
                    _ => break 'orelse___2757
                };
                t___8590 = t___8589.clone();
                break 'ok___17440;
            }
            t___8590 = panic!();
        }
        let s__1218: std::sync::Arc<String> = t___8590.to_string();
        let mut t___15994: bool = Some(s__1218.as_str()) == Some("UPDATE posts SET title = 'Updated' WHERE post_id = 99");
        #[derive(Clone)]
        struct ClosureGroup___109 {
            s__1218: std::sync::Arc<String>
        }
        impl ClosureGroup___109 {
            fn fn__15980(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__1218.clone()));
            }
        }
        let closure_group = ClosureGroup___109 {
            s__1218: s__1218.clone()
        };
        let fn__15980 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15980())
        };
        test___95.assert(t___15994, fn__15980.clone());
        test___95.soft_fail_to_hard()
    }
    #[test]
    fn deleteSqlUsesCustomPrimaryKey__2336() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___96 = temper_std::testing::Test::new();
        let tbl__1220: TableDef = TableDef::new(csid__703("posts"), [FieldDef::new(csid__703("title"), FieldType::new(StringField::new()), false, None, false)], Some(csid__703("post_id")));
        let s__1221: std::sync::Arc<String> = delete_sql(tbl__1220.clone(), 42).to_string();
        let mut t___15967: bool = Some(s__1221.as_str()) == Some("DELETE FROM posts WHERE post_id = 42");
        #[derive(Clone)]
        struct ClosureGroup___110 {
            s__1221: std::sync::Arc<String>
        }
        impl ClosureGroup___110 {
            fn fn__15956(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__1221.clone()));
            }
        }
        let closure_group = ClosureGroup___110 {
            s__1221: s__1221.clone()
        };
        let fn__15956 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15956())
        };
        test___96.assert(t___15967, fn__15956.clone());
        test___96.soft_fail_to_hard()
    }
    #[test]
    fn deleteSqlUsesDefaultIdWhenPrimaryKeyNull__2337() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___97 = temper_std::testing::Test::new();
        let tbl__1223: TableDef = TableDef::new(csid__703("users"), [FieldDef::new(csid__703("name"), FieldType::new(StringField::new()), false, None, false)], None);
        let s__1224: std::sync::Arc<String> = delete_sql(tbl__1223.clone(), 7).to_string();
        let mut t___15954: bool = Some(s__1224.as_str()) == Some("DELETE FROM users WHERE id = 7");
        #[derive(Clone)]
        struct ClosureGroup___111 {
            s__1224: std::sync::Arc<String>
        }
        impl ClosureGroup___111 {
            fn fn__15945(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__1224.clone()));
            }
        }
        let closure_group = ClosureGroup___111 {
            s__1224: s__1224.clone()
        };
        let fn__15945 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15945())
        };
        test___97.assert(t___15954, fn__15945.clone());
        test___97.soft_fail_to_hard()
    }
    #[test]
    fn alreadyInvalidChangesetSkipsSubsequentValidators__2338() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___98 = temper_std::testing::Test::new();
        let params__1226: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("A".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___15919: TableDef = userTable__704();
        let mut t___15920: SafeIdentifier = csid__703("name");
        let mut t___15921: SafeIdentifier = csid__703("email");
        let cs__1227: Changeset = changeset(t___15919.clone(), params__1226.clone()).cast(std::sync::Arc::new(vec![t___15920.clone(), t___15921.clone()])).validate_length(csid__703("name"), 3, 50).validate_required(std::sync::Arc::new(vec![csid__703("name"), csid__703("email")])).validate_contains(csid__703("email"), std::sync::Arc::new("@".to_string()));
        let mut t___15932: bool = ! cs__1227.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___112 {}
        impl ClosureGroup___112 {
            fn fn__15915(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid from validateLength".to_string());
            }
        }
        let closure_group = ClosureGroup___112 {};
        let fn__15915 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15915())
        };
        test___98.assert(t___15932, fn__15915.clone());
        let mut t___15937: bool = Some(temper_core::ListedTrait::len( & cs__1227.errors())) == Some(1);
        #[derive(Clone)]
        struct ClosureGroup___113 {
            cs__1227: Changeset
        }
        impl ClosureGroup___113 {
            fn fn__15914(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should have exactly 1 error, not accumulate: {}", temper_core::ListedTrait::len( & self.cs__1227.errors())));
            }
        }
        let closure_group = ClosureGroup___113 {
            cs__1227: cs__1227.clone()
        };
        let fn__15914 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15914())
        };
        test___98.assert(t___15937, fn__15914.clone());
        let mut t___15943: bool = Some(temper_core::ListedTrait::get( & cs__1227.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___114 {}
        impl ClosureGroup___114 {
            fn fn__15913(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error should be on name".to_string());
            }
        }
        let closure_group = ClosureGroup___114 {};
        let fn__15913 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15913())
        };
        test___98.assert(t___15943, fn__15913.clone());
        test___98.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberLessThanOrEqualPassesAtBoundary__2339() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___99 = temper_std::testing::Test::new();
        let params__1229: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("10.0".to_string()))]);
        let mut t___15901: TableDef = userTable__704();
        let mut t___15902: SafeIdentifier = csid__703("score");
        let cs__1230: Changeset = changeset(t___15901.clone(), params__1229.clone()).cast(std::sync::Arc::new(vec![t___15902.clone()])).validate_number(csid__703("score"), NumberValidationOpts::new(None, None, None, Some(10.0f64), None));
        let mut t___15907: bool = cs__1230.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___115 {}
        impl ClosureGroup___115 {
            fn fn__15898(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("10.0 <= 10.0 should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___115 {};
        let fn__15898 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15898())
        };
        test___99.assert(t___15907, fn__15898.clone());
        test___99.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberLessThanOrEqualFailsAboveBoundary__2340() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___100 = temper_std::testing::Test::new();
        let params__1232: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("10.1".to_string()))]);
        let mut t___15882: TableDef = userTable__704();
        let mut t___15883: SafeIdentifier = csid__703("score");
        let cs__1233: Changeset = changeset(t___15882.clone(), params__1232.clone()).cast(std::sync::Arc::new(vec![t___15883.clone()])).validate_number(csid__703("score"), NumberValidationOpts::new(None, None, None, Some(10.0f64), None));
        let mut t___15890: bool = ! cs__1233.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___116 {}
        impl ClosureGroup___116 {
            fn fn__15879(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("10.1 <= 10.0 should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___116 {};
        let fn__15879 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15879())
        };
        test___100.assert(t___15890, fn__15879.clone());
        let mut t___15896: bool = Some(temper_core::ListedTrait::get( & cs__1233.errors(), 0).message().as_str()) == Some("must be less than or equal to 10.0");
        #[derive(Clone)]
        struct ClosureGroup___117 {}
        impl ClosureGroup___117 {
            fn fn__15878(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("correct message".to_string());
            }
        }
        let closure_group = ClosureGroup___117 {};
        let fn__15878 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15878())
        };
        test___100.assert(t___15896, fn__15878.clone());
        test___100.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberEqualToPassesWhenEqual__2341() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___101 = temper_std::testing::Test::new();
        let params__1235: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("42.0".to_string()))]);
        let mut t___15869: TableDef = userTable__704();
        let mut t___15870: SafeIdentifier = csid__703("score");
        let cs__1236: Changeset = changeset(t___15869.clone(), params__1235.clone()).cast(std::sync::Arc::new(vec![t___15870.clone()])).validate_number(csid__703("score"), NumberValidationOpts::new(None, None, None, None, Some(42.0f64)));
        let mut t___15875: bool = cs__1236.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___118 {}
        impl ClosureGroup___118 {
            fn fn__15866(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("42.0 == 42.0 should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___118 {};
        let fn__15866 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15866())
        };
        test___101.assert(t___15875, fn__15866.clone());
        test___101.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberEqualToFailsWhenNotEqual__2342() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___102 = temper_std::testing::Test::new();
        let params__1238: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("41.9".to_string()))]);
        let mut t___15850: TableDef = userTable__704();
        let mut t___15851: SafeIdentifier = csid__703("score");
        let cs__1239: Changeset = changeset(t___15850.clone(), params__1238.clone()).cast(std::sync::Arc::new(vec![t___15851.clone()])).validate_number(csid__703("score"), NumberValidationOpts::new(None, None, None, None, Some(42.0f64)));
        let mut t___15858: bool = ! cs__1239.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___119 {}
        impl ClosureGroup___119 {
            fn fn__15847(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("41.9 == 42.0 should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___119 {};
        let fn__15847 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15847())
        };
        test___102.assert(t___15858, fn__15847.clone());
        let mut t___15864: bool = Some(temper_core::ListedTrait::get( & cs__1239.errors(), 0).message().as_str()) == Some("must be equal to 42.0");
        #[derive(Clone)]
        struct ClosureGroup___120 {}
        impl ClosureGroup___120 {
            fn fn__15846(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("correct message".to_string());
            }
        }
        let closure_group = ClosureGroup___120 {};
        let fn__15846 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15846())
        };
        test___102.assert(t___15864, fn__15846.clone());
        test___102.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberGreaterThanFailsAtExactThreshold__2343() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___103 = temper_std::testing::Test::new();
        let params__1241: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("18".to_string()))]);
        let mut t___15836: TableDef = userTable__704();
        let mut t___15837: SafeIdentifier = csid__703("age");
        let cs__1242: Changeset = changeset(t___15836.clone(), params__1241.clone()).cast(std::sync::Arc::new(vec![t___15837.clone()])).validate_number(csid__703("age"), NumberValidationOpts::new(Some(18.0f64), None, None, None, None));
        let mut t___15844: bool = ! cs__1242.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___121 {}
        impl ClosureGroup___121 {
            fn fn__15833(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("18 > 18 should fail (strict greater than)".to_string());
            }
        }
        let closure_group = ClosureGroup___121 {};
        let fn__15833 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15833())
        };
        test___103.assert(t___15844, fn__15833.clone());
        test___103.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberLessThanFailsAtExactThreshold__2344() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___104 = temper_std::testing::Test::new();
        let params__1244: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("10.0".to_string()))]);
        let mut t___15823: TableDef = userTable__704();
        let mut t___15824: SafeIdentifier = csid__703("score");
        let cs__1245: Changeset = changeset(t___15823.clone(), params__1244.clone()).cast(std::sync::Arc::new(vec![t___15824.clone()])).validate_number(csid__703("score"), NumberValidationOpts::new(None, Some(10.0f64), None, None, None));
        let mut t___15831: bool = ! cs__1245.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___122 {}
        impl ClosureGroup___122 {
            fn fn__15820(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("10.0 < 10.0 should fail (strict less than)".to_string());
            }
        }
        let closure_group = ClosureGroup___122 {};
        let fn__15820 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15820())
        };
        test___104.assert(t___15831, fn__15820.clone());
        test___104.soft_fail_to_hard()
    }
    #[test]
    fn validateFloatFailsForNonFloatString__2345() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___105 = temper_std::testing::Test::new();
        let params__1247: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("abc".to_string()))]);
        let mut t___15805: TableDef = userTable__704();
        let mut t___15806: SafeIdentifier = csid__703("score");
        let cs__1248: Changeset = changeset(t___15805.clone(), params__1247.clone()).cast(std::sync::Arc::new(vec![t___15806.clone()])).validate_float(csid__703("score"));
        let mut t___15812: bool = ! cs__1248.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___123 {}
        impl ClosureGroup___123 {
            fn fn__15802(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("abc should not parse as float".to_string());
            }
        }
        let closure_group = ClosureGroup___123 {};
        let fn__15802 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15802())
        };
        test___105.assert(t___15812, fn__15802.clone());
        let mut t___15818: bool = Some(temper_core::ListedTrait::get( & cs__1248.errors(), 0).message().as_str()) == Some("must be a number");
        #[derive(Clone)]
        struct ClosureGroup___124 {}
        impl ClosureGroup___124 {
            fn fn__15801(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("correct message".to_string());
            }
        }
        let closure_group = ClosureGroup___124 {};
        let fn__15801 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15801())
        };
        test___105.assert(t___15818, fn__15801.clone());
        test___105.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlWithAllSixFieldTypes__2346() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___106 = temper_std::testing::Test::new();
        let mut t___8416: SqlFragment;
        let mut t___8417: SqlFragment;
        let tbl__1250: TableDef = TableDef::new(csid__703("records"), [FieldDef::new(csid__703("name"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("count"), FieldType::new(IntField::new()), false, None, false), FieldDef::new(csid__703("big_id"), FieldType::new(Int64Field::new()), false, None, false), FieldDef::new(csid__703("rating"), FieldType::new(FloatField::new()), false, None, false), FieldDef::new(csid__703("active"), FieldType::new(BoolField::new()), false, None, false), FieldDef::new(csid__703("birthday"), FieldType::new(DateField::new()), false, None, false)], None);
        let params__1251: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("count".to_string()), std::sync::Arc::new("42".to_string())), (std::sync::Arc::new("big_id".to_string()), std::sync::Arc::new("9999999999".to_string())), (std::sync::Arc::new("rating".to_string()), std::sync::Arc::new("3.14".to_string())), (std::sync::Arc::new("active".to_string()), std::sync::Arc::new("true".to_string())), (std::sync::Arc::new("birthday".to_string()), std::sync::Arc::new("2000-01-15".to_string()))]);
        let mut t___15769: SafeIdentifier = csid__703("name");
        let mut t___15770: SafeIdentifier = csid__703("count");
        let mut t___15771: SafeIdentifier = csid__703("big_id");
        let mut t___15772: SafeIdentifier = csid__703("rating");
        let mut t___15773: SafeIdentifier = csid__703("active");
        let mut t___15774: SafeIdentifier = csid__703("birthday");
        let cs__1252: Changeset = changeset(tbl__1250.clone(), params__1251.clone()).cast(std::sync::Arc::new(vec![t___15769.clone(), t___15770.clone(), t___15771.clone(), t___15772.clone(), t___15773.clone(), t___15774.clone()]));
        'ok___17441: {
            'orelse___2758: {
                t___8416 = match cs__1252.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2758
                };
                t___8417 = t___8416.clone();
                break 'ok___17441;
            }
            t___8417 = panic!();
        }
        let s__1253: std::sync::Arc<String> = t___8417.to_string();
        let mut t___15778: bool = temper_core::string::index_of( & s__1253, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___125 {
            s__1253: std::sync::Arc<String>
        }
        impl ClosureGroup___125 {
            fn fn__15741(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("string field: {}", self.s__1253.clone()));
            }
        }
        let closure_group = ClosureGroup___125 {
            s__1253: s__1253.clone()
        };
        let fn__15741 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15741())
        };
        test___106.assert(t___15778, fn__15741.clone());
        let mut t___15782: bool = temper_core::string::index_of( & s__1253, "42", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___126 {
            s__1253: std::sync::Arc<String>
        }
        impl ClosureGroup___126 {
            fn fn__15740(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("int field: {}", self.s__1253.clone()));
            }
        }
        let closure_group = ClosureGroup___126 {
            s__1253: s__1253.clone()
        };
        let fn__15740 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15740())
        };
        test___106.assert(t___15782, fn__15740.clone());
        let mut t___15786: bool = temper_core::string::index_of( & s__1253, "9999999999", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___127 {
            s__1253: std::sync::Arc<String>
        }
        impl ClosureGroup___127 {
            fn fn__15739(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("int64 field: {}", self.s__1253.clone()));
            }
        }
        let closure_group = ClosureGroup___127 {
            s__1253: s__1253.clone()
        };
        let fn__15739 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15739())
        };
        test___106.assert(t___15786, fn__15739.clone());
        let mut t___15790: bool = temper_core::string::index_of( & s__1253, "3.14", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___128 {
            s__1253: std::sync::Arc<String>
        }
        impl ClosureGroup___128 {
            fn fn__15738(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("float field: {}", self.s__1253.clone()));
            }
        }
        let closure_group = ClosureGroup___128 {
            s__1253: s__1253.clone()
        };
        let fn__15738 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15738())
        };
        test___106.assert(t___15790, fn__15738.clone());
        let mut t___15794: bool = temper_core::string::index_of( & s__1253, "TRUE", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___129 {
            s__1253: std::sync::Arc<String>
        }
        impl ClosureGroup___129 {
            fn fn__15737(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("bool field: {}", self.s__1253.clone()));
            }
        }
        let closure_group = ClosureGroup___129 {
            s__1253: s__1253.clone()
        };
        let fn__15737 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15737())
        };
        test___106.assert(t___15794, fn__15737.clone());
        let mut t___15798: bool = temper_core::string::index_of( & s__1253, "'2000-01-15'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___130 {
            s__1253: std::sync::Arc<String>
        }
        impl ClosureGroup___130 {
            fn fn__15736(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("date field: {}", self.s__1253.clone()));
            }
        }
        let closure_group = ClosureGroup___130 {
            s__1253: s__1253.clone()
        };
        let fn__15736 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15736())
        };
        test___106.assert(t___15798, fn__15736.clone());
        test___106.soft_fail_to_hard()
    }
    #[test]
    fn deleteChangeOnNonNullableFieldCausesToInsertSqlToBubble__2347() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___107 = temper_std::testing::Test::new();
        let tbl__1255: TableDef = TableDef::new(csid__703("users"), [FieldDef::new(csid__703("name"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("email"), FieldType::new(StringField::new()), false, None, false)], None);
        let params__1256: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@b.com".to_string()))]);
        let mut t___15729: SafeIdentifier = csid__703("name");
        let mut t___15730: SafeIdentifier = csid__703("email");
        let cs__1257: Changeset = changeset(tbl__1255.clone(), params__1256.clone()).cast(std::sync::Arc::new(vec![t___15729.clone(), t___15730.clone()])).delete_change(csid__703("email"));
        let didBubble__1258: bool;
        'ok___17442: {
            'orelse___2759: {
                match cs__1257.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2759
                };
                didBubble__1258 = false;
                break 'ok___17442;
            }
            didBubble__1258 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___131 {}
        impl ClosureGroup___131 {
            fn fn__15717(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("removing non-nullable field should make toInsertSql bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___131 {};
        let fn__15717 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15717())
        };
        test___107.assert(didBubble__1258, fn__15717.clone());
        test___107.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthPassesAtExactMin__2348() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___108 = temper_std::testing::Test::new();
        let params__1260: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("abc".to_string()))]);
        let mut t___15709: TableDef = userTable__704();
        let mut t___15710: SafeIdentifier = csid__703("name");
        let cs__1261: Changeset = changeset(t___15709.clone(), params__1260.clone()).cast(std::sync::Arc::new(vec![t___15710.clone()])).validate_length(csid__703("name"), 3, 10);
        let mut t___15714: bool = cs__1261.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___132 {}
        impl ClosureGroup___132 {
            fn fn__15706(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("length 3 should pass for min 3".to_string());
            }
        }
        let closure_group = ClosureGroup___132 {};
        let fn__15706 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15706())
        };
        test___108.assert(t___15714, fn__15706.clone());
        test___108.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthPassesAtExactMax__2349() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___109 = temper_std::testing::Test::new();
        let params__1263: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("abcdefghij".to_string()))]);
        let mut t___15698: TableDef = userTable__704();
        let mut t___15699: SafeIdentifier = csid__703("name");
        let cs__1264: Changeset = changeset(t___15698.clone(), params__1263.clone()).cast(std::sync::Arc::new(vec![t___15699.clone()])).validate_length(csid__703("name"), 1, 10);
        let mut t___15703: bool = cs__1264.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___133 {}
        impl ClosureGroup___133 {
            fn fn__15695(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("length 10 should pass for max 10".to_string());
            }
        }
        let closure_group = ClosureGroup___133 {};
        let fn__15695 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15695())
        };
        test___109.assert(t___15703, fn__15695.clone());
        test___109.soft_fail_to_hard()
    }
    #[test]
    fn validateAcceptanceSkipsWhenFieldNotInChanges__2350() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___110 = temper_std::testing::Test::new();
        let params__1266: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___15687: TableDef = userTable__704();
        let mut t___15688: SafeIdentifier = csid__703("active");
        let cs__1267: Changeset = changeset(t___15687.clone(), params__1266.clone()).cast(std::sync::Arc::new(vec![t___15688.clone()])).validate_acceptance(csid__703("active"));
        let mut t___15692: bool = cs__1267.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___134 {}
        impl ClosureGroup___134 {
            fn fn__15685(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___134 {};
        let fn__15685 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15685())
        };
        test___110.assert(t___15692, fn__15685.clone());
        test___110.soft_fail_to_hard()
    }
    #[test]
    fn multipleValidatorsChainCorrectlyOnValidChangeset__2351() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___111 = temper_std::testing::Test::new();
        let params__1269: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___15660: TableDef = userTable__704();
        let mut t___15661: SafeIdentifier = csid__703("name");
        let mut t___15662: SafeIdentifier = csid__703("email");
        let mut t___15663: SafeIdentifier = csid__703("age");
        let cs__1270: Changeset = changeset(t___15660.clone(), params__1269.clone()).cast(std::sync::Arc::new(vec![t___15661.clone(), t___15662.clone(), t___15663.clone()])).validate_required(std::sync::Arc::new(vec![csid__703("name"), csid__703("email")])).validate_length(csid__703("name"), 2, 50).validate_contains(csid__703("email"), std::sync::Arc::new("@".to_string())).validate_int(csid__703("age")).validate_number(csid__703("age"), NumberValidationOpts::new(Some(0.0f64), Some(150.0f64), None, None, None));
        let mut t___15677: bool = cs__1270.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___135 {}
        impl ClosureGroup___135 {
            fn fn__15655(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("all validators should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___135 {};
        let fn__15655 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15655())
        };
        test___111.assert(t___15677, fn__15655.clone());
        let mut t___15683: bool = Some(temper_core::ListedTrait::len( & cs__1270.errors())) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___136 {}
        impl ClosureGroup___136 {
            fn fn__15654(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("no errors expected".to_string());
            }
        }
        let closure_group = ClosureGroup___136 {};
        let fn__15654 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15654())
        };
        test___111.assert(t___15683, fn__15654.clone());
        test___111.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlWithMultipleNonVirtualFields__2352() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___112 = temper_std::testing::Test::new();
        let mut t___8297: SqlFragment;
        let mut t___8298: SqlFragment;
        let tbl__1272: TableDef = TableDef::new(csid__703("users"), [FieldDef::new(csid__703("name"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("email"), FieldType::new(StringField::new()), false, None, false)], None);
        let params__1273: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bob@example.com".to_string()))]);
        let mut t___15638: SafeIdentifier = csid__703("name");
        let mut t___15639: SafeIdentifier = csid__703("email");
        let cs__1274: Changeset = changeset(tbl__1272.clone(), params__1273.clone()).cast(std::sync::Arc::new(vec![t___15638.clone(), t___15639.clone()]));
        'ok___17443: {
            'orelse___2760: {
                t___8297 = match cs__1274.to_update_sql(5) {
                    Ok(x) => x,
                    _ => break 'orelse___2760
                };
                t___8298 = t___8297.clone();
                break 'ok___17443;
            }
            t___8298 = panic!();
        }
        let s__1275: std::sync::Arc<String> = t___8298.to_string();
        let mut t___15643: bool = temper_core::string::index_of( & s__1275, "name = 'Bob'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___137 {
            s__1275: std::sync::Arc<String>
        }
        impl ClosureGroup___137 {
            fn fn__15626(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("name in SET: {}", self.s__1275.clone()));
            }
        }
        let closure_group = ClosureGroup___137 {
            s__1275: s__1275.clone()
        };
        let fn__15626 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15626())
        };
        test___112.assert(t___15643, fn__15626.clone());
        let mut t___15647: bool = temper_core::string::index_of( & s__1275, "email = 'bob@example.com'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___138 {
            s__1275: std::sync::Arc<String>
        }
        impl ClosureGroup___138 {
            fn fn__15625(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("email in SET: {}", self.s__1275.clone()));
            }
        }
        let closure_group = ClosureGroup___138 {
            s__1275: s__1275.clone()
        };
        let fn__15625 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15625())
        };
        test___112.assert(t___15647, fn__15625.clone());
        let mut t___15651: bool = temper_core::string::index_of( & s__1275, "WHERE id = 5", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___139 {
            s__1275: std::sync::Arc<String>
        }
        impl ClosureGroup___139 {
            fn fn__15624(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("WHERE clause: {}", self.s__1275.clone()));
            }
        }
        let closure_group = ClosureGroup___139 {
            s__1275: s__1275.clone()
        };
        let fn__15624 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15624())
        };
        test___112.assert(t___15651, fn__15624.clone());
        test___112.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlBubblesWhenAllChangesAreVirtualFields__2353() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___113 = temper_std::testing::Test::new();
        let tbl__1277: TableDef = TableDef::new(csid__703("users"), [FieldDef::new(csid__703("name"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__703("computed"), FieldType::new(StringField::new()), true, None, true)], None);
        let params__1278: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("computed".to_string()), std::sync::Arc::new("derived".to_string()))]);
        let mut t___15620: SafeIdentifier = csid__703("computed");
        let cs__1279: Changeset = changeset(tbl__1277.clone(), params__1278.clone()).cast(std::sync::Arc::new(vec![t___15620.clone()]));
        let didBubble__1280: bool;
        'ok___17444: {
            'orelse___2761: {
                match cs__1279.to_update_sql(1) {
                    Ok(x) => x,
                    _ => break 'orelse___2761
                };
                didBubble__1280 = false;
                break 'ok___17444;
            }
            didBubble__1280 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___140 {}
        impl ClosureGroup___140 {
            fn fn__15608(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should bubble when all changes are virtual".to_string());
            }
        }
        let closure_group = ClosureGroup___140 {};
        let fn__15608 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15608())
        };
        test___113.assert(didBubble__1280, fn__15608.clone());
        test___113.soft_fail_to_hard()
    }
    #[test]
    fn putChangeSatisfiesSubsequentValidateRequired__2354() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___114 = temper_std::testing::Test::new();
        let params__1282: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___15598: TableDef = userTable__704();
        let mut t___15599: SafeIdentifier = csid__703("name");
        let cs__1283: Changeset = changeset(t___15598.clone(), params__1282.clone()).cast(std::sync::Arc::new(vec![t___15599.clone()])).put_change(csid__703("name"), std::sync::Arc::new("Injected".to_string())).validate_required(std::sync::Arc::new(vec![csid__703("name")]));
        let mut t___15605: bool = cs__1283.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___141 {}
        impl ClosureGroup___141 {
            fn fn__15596(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("putChange should satisfy required".to_string());
            }
        }
        let closure_group = ClosureGroup___141 {};
        let fn__15596 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15596())
        };
        test___114.assert(t___15605, fn__15596.clone());
        test___114.soft_fail_to_hard()
    }
    #[test]
    fn validateStartsWithSkipsWhenFieldNotInChanges__2355() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___115 = temper_std::testing::Test::new();
        let params__1285: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___15588: TableDef = userTable__704();
        let mut t___15589: SafeIdentifier = csid__703("name");
        let cs__1286: Changeset = changeset(t___15588.clone(), params__1285.clone()).cast(std::sync::Arc::new(vec![t___15589.clone()])).validate_starts_with(csid__703("name"), std::sync::Arc::new("Dr.".to_string()));
        let mut t___15593: bool = cs__1286.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___142 {}
        impl ClosureGroup___142 {
            fn fn__15586(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___142 {};
        let fn__15586 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15586())
        };
        test___115.assert(t___15593, fn__15586.clone());
        test___115.soft_fail_to_hard()
    }
    #[test]
    fn validateEndsWithSkipsWhenFieldNotInChanges__2356() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___116 = temper_std::testing::Test::new();
        let params__1288: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___15578: TableDef = userTable__704();
        let mut t___15579: SafeIdentifier = csid__703("name");
        let cs__1289: Changeset = changeset(t___15578.clone(), params__1288.clone()).cast(std::sync::Arc::new(vec![t___15579.clone()])).validate_ends_with(csid__703("name"), std::sync::Arc::new(".com".to_string()));
        let mut t___15583: bool = cs__1289.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___143 {}
        impl ClosureGroup___143 {
            fn fn__15576(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___143 {};
        let fn__15576 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15576())
        };
        test___116.assert(t___15583, fn__15576.clone());
        test___116.soft_fail_to_hard()
    }
    #[test]
    fn validateIntAcceptsZero__2357() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___117 = temper_std::testing::Test::new();
        let params__1291: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("0".to_string()))]);
        let mut t___15568: TableDef = userTable__704();
        let mut t___15569: SafeIdentifier = csid__703("age");
        let cs__1292: Changeset = changeset(t___15568.clone(), params__1291.clone()).cast(std::sync::Arc::new(vec![t___15569.clone()])).validate_int(csid__703("age"));
        let mut t___15573: bool = cs__1292.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___144 {}
        impl ClosureGroup___144 {
            fn fn__15565(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("0 should be a valid int".to_string());
            }
        }
        let closure_group = ClosureGroup___144 {};
        let fn__15565 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15565())
        };
        test___117.assert(t___15573, fn__15565.clone());
        test___117.soft_fail_to_hard()
    }
    #[test]
    fn validateIntAcceptsNegative__2358() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___118 = temper_std::testing::Test::new();
        let params__1294: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("-5".to_string()))]);
        let mut t___15557: TableDef = userTable__704();
        let mut t___15558: SafeIdentifier = csid__703("age");
        let cs__1295: Changeset = changeset(t___15557.clone(), params__1294.clone()).cast(std::sync::Arc::new(vec![t___15558.clone()])).validate_int(csid__703("age"));
        let mut t___15562: bool = cs__1295.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___145 {}
        impl ClosureGroup___145 {
            fn fn__15554(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("-5 should be a valid int".to_string());
            }
        }
        let closure_group = ClosureGroup___145 {};
        let fn__15554 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15554())
        };
        test___118.assert(t___15562, fn__15554.clone());
        test___118.soft_fail_to_hard()
    }
    #[test]
    fn changesetImmutabilityValidatorsDoNotMutateBase__2359() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___119 = temper_std::testing::Test::new();
        let params__1297: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("A".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___15538: TableDef = userTable__704();
        let mut t___15539: SafeIdentifier = csid__703("name");
        let mut t___15540: SafeIdentifier = csid__703("email");
        let base__1298: Changeset = changeset(t___15538.clone(), params__1297.clone()).cast(std::sync::Arc::new(vec![t___15539.clone(), t___15540.clone()]));
        let failed__1299: Changeset = base__1298.validate_length(csid__703("name"), 3, 50);
        let passed__1300: Changeset = base__1298.validate_required(std::sync::Arc::new(vec![csid__703("name"), csid__703("email")]));
        let mut t___15549: bool = ! failed__1299.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___146 {}
        impl ClosureGroup___146 {
            fn fn__15534(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("failed branch should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___146 {};
        let fn__15534 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15534())
        };
        test___119.assert(t___15549, fn__15534.clone());
        let mut t___15551: bool = passed__1300.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___147 {}
        impl ClosureGroup___147 {
            fn fn__15533(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("passed branch should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___147 {};
        let fn__15533 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15533())
        };
        test___119.assert(t___15551, fn__15533.clone());
        test___119.soft_fail_to_hard()
    }
    #[test]
    fn bareFromProducesSelect__2441() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___120 = temper_std::testing::Test::new();
        let q__1653: Query = from(sid__709("users"));
        let mut t___15091: bool = Some(q__1653.to_sql().to_string().as_str()) == Some("SELECT * FROM users");
        #[derive(Clone)]
        struct ClosureGroup___150 {}
        impl ClosureGroup___150 {
            fn fn__15086(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("bare query".to_string());
            }
        }
        let closure_group = ClosureGroup___150 {};
        let fn__15086 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15086())
        };
        test___120.assert(t___15091, fn__15086.clone());
        test___120.soft_fail_to_hard()
    }
    #[test]
    fn selectRestrictsColumns__2442() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___121 = temper_std::testing::Test::new();
        let mut t___15077: SafeIdentifier = sid__709("users");
        let mut t___15078: SafeIdentifier = sid__709("id");
        let mut t___15079: SafeIdentifier = sid__709("name");
        let q__1655: Query = from(t___15077.clone()).select([t___15078.clone(), t___15079.clone()]);
        let mut t___15084: bool = Some(q__1655.to_sql().to_string().as_str()) == Some("SELECT id, name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___151 {}
        impl ClosureGroup___151 {
            fn fn__15076(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("select columns".to_string());
            }
        }
        let closure_group = ClosureGroup___151 {};
        let fn__15076 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15076())
        };
        test___121.assert(t___15084, fn__15076.clone());
        test___121.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithIntValue__2443() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___122 = temper_std::testing::Test::new();
        let mut t___15065: SafeIdentifier = sid__709("users");
        let mut t___15066: SqlBuilder = SqlBuilder::new();
        t___15066.append_safe("age > ");
        t___15066.append_int32(18);
        let mut t___15069: SqlFragment = t___15066.accumulated();
        let q__1657: Query = from(t___15065.clone()).r#where(t___15069.clone());
        let mut t___15074: bool = Some(q__1657.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18");
        #[derive(Clone)]
        struct ClosureGroup___152 {}
        impl ClosureGroup___152 {
            fn fn__15064(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where int".to_string());
            }
        }
        let closure_group = ClosureGroup___152 {};
        let fn__15064 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15064())
        };
        test___122.assert(t___15074, fn__15064.clone());
        test___122.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithBoolValue__2445() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___123 = temper_std::testing::Test::new();
        let mut t___15053: SafeIdentifier = sid__709("users");
        let mut t___15054: SqlBuilder = SqlBuilder::new();
        t___15054.append_safe("active = ");
        t___15054.append_boolean(true);
        let mut t___15057: SqlFragment = t___15054.accumulated();
        let q__1659: Query = from(t___15053.clone()).r#where(t___15057.clone());
        let mut t___15062: bool = Some(q__1659.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___153 {}
        impl ClosureGroup___153 {
            fn fn__15052(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where bool".to_string());
            }
        }
        let closure_group = ClosureGroup___153 {};
        let fn__15052 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15052())
        };
        test___123.assert(t___15062, fn__15052.clone());
        test___123.soft_fail_to_hard()
    }
    #[test]
    fn chainedWhereUsesAnd__2447() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___124 = temper_std::testing::Test::new();
        let mut t___15036: SafeIdentifier = sid__709("users");
        let mut t___15037: SqlBuilder = SqlBuilder::new();
        t___15037.append_safe("age > ");
        t___15037.append_int32(18);
        let mut t___15040: SqlFragment = t___15037.accumulated();
        let mut t___15041: Query = from(t___15036.clone()).r#where(t___15040.clone());
        let mut t___15042: SqlBuilder = SqlBuilder::new();
        t___15042.append_safe("active = ");
        t___15042.append_boolean(true);
        let q__1661: Query = t___15041.r#where(t___15042.accumulated());
        let mut t___15050: bool = Some(q__1661.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___154 {}
        impl ClosureGroup___154 {
            fn fn__15035(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained where".to_string());
            }
        }
        let closure_group = ClosureGroup___154 {};
        let fn__15035 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15035())
        };
        test___124.assert(t___15050, fn__15035.clone());
        test___124.soft_fail_to_hard()
    }
    #[test]
    fn orderByAsc__2450() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___125 = temper_std::testing::Test::new();
        let mut t___15027: SafeIdentifier = sid__709("users");
        let mut t___15028: SafeIdentifier = sid__709("name");
        let q__1663: Query = from(t___15027.clone()).order_by(t___15028.clone(), true);
        let mut t___15033: bool = Some(q__1663.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC");
        #[derive(Clone)]
        struct ClosureGroup___155 {}
        impl ClosureGroup___155 {
            fn fn__15026(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order asc".to_string());
            }
        }
        let closure_group = ClosureGroup___155 {};
        let fn__15026 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15026())
        };
        test___125.assert(t___15033, fn__15026.clone());
        test___125.soft_fail_to_hard()
    }
    #[test]
    fn orderByDesc__2451() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___126 = temper_std::testing::Test::new();
        let mut t___15018: SafeIdentifier = sid__709("users");
        let mut t___15019: SafeIdentifier = sid__709("created_at");
        let q__1665: Query = from(t___15018.clone()).order_by(t___15019.clone(), false);
        let mut t___15024: bool = Some(q__1665.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY created_at DESC");
        #[derive(Clone)]
        struct ClosureGroup___156 {}
        impl ClosureGroup___156 {
            fn fn__15017(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order desc".to_string());
            }
        }
        let closure_group = ClosureGroup___156 {};
        let fn__15017 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15017())
        };
        test___126.assert(t___15024, fn__15017.clone());
        test___126.soft_fail_to_hard()
    }
    #[test]
    fn limitAndOffset__2452() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___127 = temper_std::testing::Test::new();
        let mut t___7677: Query;
        let mut t___7678: Query;
        let q__1667: Query;
        'ok___17446: {
            'orelse___2763: {
                t___7677 = match from(sid__709("users")).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___2763
                };
                t___7678 = match t___7677.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___2763
                };
                q__1667 = t___7678.clone();
                break 'ok___17446;
            }
            q__1667 = panic!();
        }
        let mut t___15015: bool = Some(q__1667.to_sql().to_string().as_str()) == Some("SELECT * FROM users LIMIT 10 OFFSET 20");
        #[derive(Clone)]
        struct ClosureGroup___157 {}
        impl ClosureGroup___157 {
            fn fn__15010(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("limit/offset".to_string());
            }
        }
        let closure_group = ClosureGroup___157 {};
        let fn__15010 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15010())
        };
        test___127.assert(t___15015, fn__15010.clone());
        test___127.soft_fail_to_hard()
    }
    #[test]
    fn limitBubblesOnNegative__2453() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___128 = temper_std::testing::Test::new();
        let didBubble__1669: bool;
        'ok___17447: {
            'orelse___2764: {
                match from(sid__709("users")).limit(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___2764
                };
                didBubble__1669 = false;
                break 'ok___17447;
            }
            didBubble__1669 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___158 {}
        impl ClosureGroup___158 {
            fn fn__15006(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative limit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___158 {};
        let fn__15006 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15006())
        };
        test___128.assert(didBubble__1669, fn__15006.clone());
        test___128.soft_fail_to_hard()
    }
    #[test]
    fn offsetBubblesOnNegative__2454() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___129 = temper_std::testing::Test::new();
        let didBubble__1671: bool;
        'ok___17448: {
            'orelse___2765: {
                match from(sid__709("users")).offset(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___2765
                };
                didBubble__1671 = false;
                break 'ok___17448;
            }
            didBubble__1671 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___159 {}
        impl ClosureGroup___159 {
            fn fn__15002(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative offset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___159 {};
        let fn__15002 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15002())
        };
        test___129.assert(didBubble__1671, fn__15002.clone());
        test___129.soft_fail_to_hard()
    }
    #[test]
    fn complexComposedQuery__2455() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___130 = temper_std::testing::Test::new();
        let mut t___14980: SafeIdentifier;
        let mut t___14981: SafeIdentifier;
        let mut t___14982: SafeIdentifier;
        let mut t___14983: SafeIdentifier;
        let mut t___14984: Query;
        let mut t___14985: SqlBuilder;
        let mut t___14989: Query;
        let mut t___14990: SqlBuilder;
        let mut t___7663: Query;
        let mut t___7664: Query;
        let minAge__1673: i32 = 21;
        let q__1674: Query;
        'ok___17449: {
            'orelse___2766: {
                t___14980 = sid__709("users");
                t___14981 = sid__709("id");
                t___14982 = sid__709("name");
                t___14983 = sid__709("email");
                t___14984 = from(t___14980.clone()).select([t___14981.clone(), t___14982.clone(), t___14983.clone()]);
                t___14985 = SqlBuilder::new();
                t___14985.append_safe("age >= ");
                t___14985.append_int32(21);
                t___14989 = t___14984.r#where(t___14985.accumulated());
                t___14990 = SqlBuilder::new();
                t___14990.append_safe("active = ");
                t___14990.append_boolean(true);
                t___7663 = match t___14989.r#where(t___14990.accumulated()).order_by(sid__709("name"), true).limit(25) {
                    Ok(x) => x,
                    _ => break 'orelse___2766
                };
                t___7664 = match t___7663.offset(0) {
                    Ok(x) => x,
                    _ => break 'orelse___2766
                };
                q__1674 = t___7664.clone();
                break 'ok___17449;
            }
            q__1674 = panic!();
        }
        let mut t___15000: bool = Some(q__1674.to_sql().to_string().as_str()) == Some("SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0");
        #[derive(Clone)]
        struct ClosureGroup___160 {}
        impl ClosureGroup___160 {
            fn fn__14979(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("complex query".to_string());
            }
        }
        let closure_group = ClosureGroup___160 {};
        let fn__14979 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14979())
        };
        test___130.assert(t___15000, fn__14979.clone());
        test___130.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlAppliesDefaultLimitWhenNoneSet__2458() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___131 = temper_std::testing::Test::new();
        let mut t___7640: SqlFragment;
        let mut t___7641: SqlFragment;
        let q__1676: Query = from(sid__709("users"));
        'ok___17450: {
            'orelse___2767: {
                t___7640 = match q__1676.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___2767
                };
                t___7641 = t___7640.clone();
                break 'ok___17450;
            }
            t___7641 = panic!();
        }
        let s__1677: std::sync::Arc<String> = t___7641.to_string();
        let mut t___14977: bool = Some(s__1677.as_str()) == Some("SELECT * FROM users LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___161 {
            s__1677: std::sync::Arc<String>
        }
        impl ClosureGroup___161 {
            fn fn__14973(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should have limit: {}", self.s__1677.clone()));
            }
        }
        let closure_group = ClosureGroup___161 {
            s__1677: s__1677.clone()
        };
        let fn__14973 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14973())
        };
        test___131.assert(t___14977, fn__14973.clone());
        test___131.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlRespectsExplicitLimit__2459() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___132 = temper_std::testing::Test::new();
        let mut t___7632: Query;
        let mut t___7635: SqlFragment;
        let mut t___7636: SqlFragment;
        let q__1679: Query;
        'ok___17451: {
            'orelse___2768: {
                t___7632 = match from(sid__709("users")).limit(5) {
                    Ok(x) => x,
                    _ => break 'orelse___2768
                };
                q__1679 = t___7632.clone();
                break 'ok___17451;
            }
            q__1679 = panic!();
        }
        'ok___17452: {
            'orelse___2769: {
                t___7635 = match q__1679.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___2769
                };
                t___7636 = t___7635.clone();
                break 'ok___17452;
            }
            t___7636 = panic!();
        }
        let s__1680: std::sync::Arc<String> = t___7636.to_string();
        let mut t___14971: bool = Some(s__1680.as_str()) == Some("SELECT * FROM users LIMIT 5");
        #[derive(Clone)]
        struct ClosureGroup___162 {
            s__1680: std::sync::Arc<String>
        }
        impl ClosureGroup___162 {
            fn fn__14967(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("explicit limit preserved: {}", self.s__1680.clone()));
            }
        }
        let closure_group = ClosureGroup___162 {
            s__1680: s__1680.clone()
        };
        let fn__14967 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14967())
        };
        test___132.assert(t___14971, fn__14967.clone());
        test___132.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlBubblesOnNegativeDefaultLimit__2460() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___133 = temper_std::testing::Test::new();
        let didBubble__1682: bool;
        'ok___17453: {
            'orelse___2770: {
                match from(sid__709("users")).safe_to_sql(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___2770
                };
                didBubble__1682 = false;
                break 'ok___17453;
            }
            didBubble__1682 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___163 {}
        impl ClosureGroup___163 {
            fn fn__14963(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative defaultLimit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___163 {};
        let fn__14963 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14963())
        };
        test___133.assert(didBubble__1682, fn__14963.clone());
        test___133.soft_fail_to_hard()
    }
    #[test]
    fn whereWithInjectionAttemptInStringValueIsEscaped__2461() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___134 = temper_std::testing::Test::new();
        let evil__1684: std::sync::Arc<String> = std::sync::Arc::new("'; DROP TABLE users; --".to_string());
        let mut t___14947: SafeIdentifier = sid__709("users");
        let mut t___14948: SqlBuilder = SqlBuilder::new();
        t___14948.append_safe("name = ");
        t___14948.append_string("'; DROP TABLE users; --");
        let mut t___14951: SqlFragment = t___14948.accumulated();
        let q__1685: Query = from(t___14947.clone()).r#where(t___14951.clone());
        let s__1686: std::sync::Arc<String> = q__1685.to_sql().to_string();
        let mut t___14956: bool = temper_core::string::index_of( & s__1686, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___164 {
            s__1686: std::sync::Arc<String>
        }
        impl ClosureGroup___164 {
            fn fn__14946(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("quotes must be doubled: {}", self.s__1686.clone()));
            }
        }
        let closure_group = ClosureGroup___164 {
            s__1686: s__1686.clone()
        };
        let fn__14946 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14946())
        };
        test___134.assert(t___14956, fn__14946.clone());
        let mut t___14960: bool = temper_core::string::index_of( & s__1686, "SELECT * FROM users WHERE name =", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___165 {
            s__1686: std::sync::Arc<String>
        }
        impl ClosureGroup___165 {
            fn fn__14945(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("structure intact: {}", self.s__1686.clone()));
            }
        }
        let closure_group = ClosureGroup___165 {
            s__1686: s__1686.clone()
        };
        let fn__14945 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14945())
        };
        test___134.assert(t___14960, fn__14945.clone());
        test___134.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__2463() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___135 = temper_std::testing::Test::new();
        let attack__1688: std::sync::Arc<String> = std::sync::Arc::new("users; DROP TABLE users; --".to_string());
        let didBubble__1689: bool;
        'ok___17454: {
            'orelse___2771: {
                match safe_identifier("users; DROP TABLE users; --") {
                    Ok(x) => x,
                    _ => break 'orelse___2771
                };
                didBubble__1689 = false;
                break 'ok___17454;
            }
            didBubble__1689 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___166 {}
        impl ClosureGroup___166 {
            fn fn__14942(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("metacharacter-containing name must be rejected at construction".to_string());
            }
        }
        let closure_group = ClosureGroup___166 {};
        let fn__14942 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14942())
        };
        test___135.assert(didBubble__1689, fn__14942.clone());
        test___135.soft_fail_to_hard()
    }
    #[test]
    fn innerJoinProducesInnerJoin__2464() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___136 = temper_std::testing::Test::new();
        let mut t___14931: SafeIdentifier = sid__709("users");
        let mut t___14932: SafeIdentifier = sid__709("orders");
        let mut t___14933: SqlBuilder = SqlBuilder::new();
        t___14933.append_safe("users.id = orders.user_id");
        let mut t___14935: SqlFragment = t___14933.accumulated();
        let q__1691: Query = from(t___14931.clone()).inner_join(t___14932.clone(), t___14935.clone());
        let mut t___14940: bool = Some(q__1691.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___167 {}
        impl ClosureGroup___167 {
            fn fn__14930(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("inner join".to_string());
            }
        }
        let closure_group = ClosureGroup___167 {};
        let fn__14930 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14930())
        };
        test___136.assert(t___14940, fn__14930.clone());
        test___136.soft_fail_to_hard()
    }
    #[test]
    fn leftJoinProducesLeftJoin__2466() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___137 = temper_std::testing::Test::new();
        let mut t___14919: SafeIdentifier = sid__709("users");
        let mut t___14920: SafeIdentifier = sid__709("profiles");
        let mut t___14921: SqlBuilder = SqlBuilder::new();
        t___14921.append_safe("users.id = profiles.user_id");
        let mut t___14923: SqlFragment = t___14921.accumulated();
        let q__1693: Query = from(t___14919.clone()).left_join(t___14920.clone(), t___14923.clone());
        let mut t___14928: bool = Some(q__1693.to_sql().to_string().as_str()) == Some("SELECT * FROM users LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___168 {}
        impl ClosureGroup___168 {
            fn fn__14918(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("left join".to_string());
            }
        }
        let closure_group = ClosureGroup___168 {};
        let fn__14918 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14918())
        };
        test___137.assert(t___14928, fn__14918.clone());
        test___137.soft_fail_to_hard()
    }
    #[test]
    fn rightJoinProducesRightJoin__2468() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___138 = temper_std::testing::Test::new();
        let mut t___14907: SafeIdentifier = sid__709("orders");
        let mut t___14908: SafeIdentifier = sid__709("users");
        let mut t___14909: SqlBuilder = SqlBuilder::new();
        t___14909.append_safe("orders.user_id = users.id");
        let mut t___14911: SqlFragment = t___14909.accumulated();
        let q__1695: Query = from(t___14907.clone()).right_join(t___14908.clone(), t___14911.clone());
        let mut t___14916: bool = Some(q__1695.to_sql().to_string().as_str()) == Some("SELECT * FROM orders RIGHT JOIN users ON orders.user_id = users.id");
        #[derive(Clone)]
        struct ClosureGroup___169 {}
        impl ClosureGroup___169 {
            fn fn__14906(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("right join".to_string());
            }
        }
        let closure_group = ClosureGroup___169 {};
        let fn__14906 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14906())
        };
        test___138.assert(t___14916, fn__14906.clone());
        test___138.soft_fail_to_hard()
    }
    #[test]
    fn fullJoinProducesFullOuterJoin__2470() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___139 = temper_std::testing::Test::new();
        let mut t___14895: SafeIdentifier = sid__709("users");
        let mut t___14896: SafeIdentifier = sid__709("orders");
        let mut t___14897: SqlBuilder = SqlBuilder::new();
        t___14897.append_safe("users.id = orders.user_id");
        let mut t___14899: SqlFragment = t___14897.accumulated();
        let q__1697: Query = from(t___14895.clone()).full_join(t___14896.clone(), t___14899.clone());
        let mut t___14904: bool = Some(q__1697.to_sql().to_string().as_str()) == Some("SELECT * FROM users FULL OUTER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___170 {}
        impl ClosureGroup___170 {
            fn fn__14894(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full join".to_string());
            }
        }
        let closure_group = ClosureGroup___170 {};
        let fn__14894 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14894())
        };
        test___139.assert(t___14904, fn__14894.clone());
        test___139.soft_fail_to_hard()
    }
    #[test]
    fn chainedJoins__2472() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___140 = temper_std::testing::Test::new();
        let mut t___14878: SafeIdentifier = sid__709("users");
        let mut t___14879: SafeIdentifier = sid__709("orders");
        let mut t___14880: SqlBuilder = SqlBuilder::new();
        t___14880.append_safe("users.id = orders.user_id");
        let mut t___14882: SqlFragment = t___14880.accumulated();
        let mut t___14883: Query = from(t___14878.clone()).inner_join(t___14879.clone(), t___14882.clone());
        let mut t___14884: SafeIdentifier = sid__709("profiles");
        let mut t___14885: SqlBuilder = SqlBuilder::new();
        t___14885.append_safe("users.id = profiles.user_id");
        let q__1699: Query = t___14883.left_join(t___14884.clone(), t___14885.accumulated());
        let mut t___14892: bool = Some(q__1699.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___171 {}
        impl ClosureGroup___171 {
            fn fn__14877(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained joins".to_string());
            }
        }
        let closure_group = ClosureGroup___171 {};
        let fn__14877 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14877())
        };
        test___140.assert(t___14892, fn__14877.clone());
        test___140.soft_fail_to_hard()
    }
    #[test]
    fn joinWithWhereAndOrderBy__2475() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___141 = temper_std::testing::Test::new();
        let mut t___14859: SafeIdentifier;
        let mut t___14860: SafeIdentifier;
        let mut t___14861: SqlBuilder;
        let mut t___14863: SqlFragment;
        let mut t___14864: Query;
        let mut t___14865: SqlBuilder;
        let mut t___7547: Query;
        let q__1701: Query;
        'ok___17455: {
            'orelse___2772: {
                t___14859 = sid__709("users");
                t___14860 = sid__709("orders");
                t___14861 = SqlBuilder::new();
                t___14861.append_safe("users.id = orders.user_id");
                t___14863 = t___14861.accumulated();
                t___14864 = from(t___14859.clone()).inner_join(t___14860.clone(), t___14863.clone());
                t___14865 = SqlBuilder::new();
                t___14865.append_safe("orders.total > ");
                t___14865.append_int32(100);
                t___7547 = match t___14864.r#where(t___14865.accumulated()).order_by(sid__709("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___2772
                };
                q__1701 = t___7547.clone();
                break 'ok___17455;
            }
            q__1701 = panic!();
        }
        let mut t___14875: bool = Some(q__1701.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100 ORDER BY name ASC LIMIT 10");
        #[derive(Clone)]
        struct ClosureGroup___172 {}
        impl ClosureGroup___172 {
            fn fn__14858(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with where/order/limit".to_string());
            }
        }
        let closure_group = ClosureGroup___172 {};
        let fn__14858 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14858())
        };
        test___141.assert(t___14875, fn__14858.clone());
        test___141.soft_fail_to_hard()
    }
    #[test]
    fn colHelperProducesQualifiedReference__2478() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___142 = temper_std::testing::Test::new();
        let c__1703: SqlFragment = col(sid__709("users"), sid__709("id"));
        let mut t___14856: bool = Some(c__1703.to_string().as_str()) == Some("users.id");
        #[derive(Clone)]
        struct ClosureGroup___173 {}
        impl ClosureGroup___173 {
            fn fn__14850(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("col helper".to_string());
            }
        }
        let closure_group = ClosureGroup___173 {};
        let fn__14850 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14850())
        };
        test___142.assert(t___14856, fn__14850.clone());
        test___142.soft_fail_to_hard()
    }
    #[test]
    fn joinWithColHelper__2479() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___143 = temper_std::testing::Test::new();
        let onCond__1705: SqlFragment = col(sid__709("users"), sid__709("id"));
        let b__1706: SqlBuilder = SqlBuilder::new();
        b__1706.append_fragment(onCond__1705.clone());
        b__1706.append_safe(" = ");
        b__1706.append_fragment(col(sid__709("orders"), sid__709("user_id")));
        let mut t___14841: SafeIdentifier = sid__709("users");
        let mut t___14842: SafeIdentifier = sid__709("orders");
        let mut t___14843: SqlFragment = b__1706.accumulated();
        let q__1707: Query = from(t___14841.clone()).inner_join(t___14842.clone(), t___14843.clone());
        let mut t___14848: bool = Some(q__1707.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___174 {}
        impl ClosureGroup___174 {
            fn fn__14830(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with col".to_string());
            }
        }
        let closure_group = ClosureGroup___174 {};
        let fn__14830 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14830())
        };
        test___143.assert(t___14848, fn__14830.clone());
        test___143.soft_fail_to_hard()
    }
    #[test]
    fn orWhereBasic__2480() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___144 = temper_std::testing::Test::new();
        let mut t___14819: SafeIdentifier = sid__709("users");
        let mut t___14820: SqlBuilder = SqlBuilder::new();
        t___14820.append_safe("status = ");
        t___14820.append_string("active");
        let mut t___14823: SqlFragment = t___14820.accumulated();
        let q__1709: Query = from(t___14819.clone()).or_where(t___14823.clone());
        let mut t___14828: bool = Some(q__1709.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE status = 'active'");
        #[derive(Clone)]
        struct ClosureGroup___175 {}
        impl ClosureGroup___175 {
            fn fn__14818(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orWhere basic".to_string());
            }
        }
        let closure_group = ClosureGroup___175 {};
        let fn__14818 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14818())
        };
        test___144.assert(t___14828, fn__14818.clone());
        test___144.soft_fail_to_hard()
    }
    #[test]
    fn whereThenOrWhere__2482() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___145 = temper_std::testing::Test::new();
        let mut t___14802: SafeIdentifier = sid__709("users");
        let mut t___14803: SqlBuilder = SqlBuilder::new();
        t___14803.append_safe("age > ");
        t___14803.append_int32(18);
        let mut t___14806: SqlFragment = t___14803.accumulated();
        let mut t___14807: Query = from(t___14802.clone()).r#where(t___14806.clone());
        let mut t___14808: SqlBuilder = SqlBuilder::new();
        t___14808.append_safe("vip = ");
        t___14808.append_boolean(true);
        let q__1711: Query = t___14807.or_where(t___14808.accumulated());
        let mut t___14816: bool = Some(q__1711.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___176 {}
        impl ClosureGroup___176 {
            fn fn__14801(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where then orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___176 {};
        let fn__14801 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14801())
        };
        test___145.assert(t___14816, fn__14801.clone());
        test___145.soft_fail_to_hard()
    }
    #[test]
    fn multipleOrWhere__2485() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___146 = temper_std::testing::Test::new();
        let mut t___14780: SafeIdentifier = sid__709("users");
        let mut t___14781: SqlBuilder = SqlBuilder::new();
        t___14781.append_safe("active = ");
        t___14781.append_boolean(true);
        let mut t___14784: SqlFragment = t___14781.accumulated();
        let mut t___14785: Query = from(t___14780.clone()).r#where(t___14784.clone());
        let mut t___14786: SqlBuilder = SqlBuilder::new();
        t___14786.append_safe("role = ");
        t___14786.append_string("admin");
        let mut t___14790: Query = t___14785.or_where(t___14786.accumulated());
        let mut t___14791: SqlBuilder = SqlBuilder::new();
        t___14791.append_safe("role = ");
        t___14791.append_string("moderator");
        let q__1713: Query = t___14790.or_where(t___14791.accumulated());
        let mut t___14799: bool = Some(q__1713.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE OR role = 'admin' OR role = 'moderator'");
        #[derive(Clone)]
        struct ClosureGroup___177 {}
        impl ClosureGroup___177 {
            fn fn__14779(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("multiple orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___177 {};
        let fn__14779 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14779())
        };
        test___146.assert(t___14799, fn__14779.clone());
        test___146.soft_fail_to_hard()
    }
    #[test]
    fn mixedWhereAndOrWhere__2489() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___147 = temper_std::testing::Test::new();
        let mut t___14758: SafeIdentifier = sid__709("users");
        let mut t___14759: SqlBuilder = SqlBuilder::new();
        t___14759.append_safe("age > ");
        t___14759.append_int32(18);
        let mut t___14762: SqlFragment = t___14759.accumulated();
        let mut t___14763: Query = from(t___14758.clone()).r#where(t___14762.clone());
        let mut t___14764: SqlBuilder = SqlBuilder::new();
        t___14764.append_safe("active = ");
        t___14764.append_boolean(true);
        let mut t___14768: Query = t___14763.r#where(t___14764.accumulated());
        let mut t___14769: SqlBuilder = SqlBuilder::new();
        t___14769.append_safe("vip = ");
        t___14769.append_boolean(true);
        let q__1715: Query = t___14768.or_where(t___14769.accumulated());
        let mut t___14777: bool = Some(q__1715.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___178 {}
        impl ClosureGroup___178 {
            fn fn__14757(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mixed where and orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___178 {};
        let fn__14757 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14757())
        };
        test___147.assert(t___14777, fn__14757.clone());
        test___147.soft_fail_to_hard()
    }
    #[test]
    fn whereNull__2493() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___148 = temper_std::testing::Test::new();
        let mut t___14749: SafeIdentifier = sid__709("users");
        let mut t___14750: SafeIdentifier = sid__709("deleted_at");
        let q__1717: Query = from(t___14749.clone()).where_null(t___14750.clone());
        let mut t___14755: bool = Some(q__1717.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___179 {}
        impl ClosureGroup___179 {
            fn fn__14748(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull".to_string());
            }
        }
        let closure_group = ClosureGroup___179 {};
        let fn__14748 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14748())
        };
        test___148.assert(t___14755, fn__14748.clone());
        test___148.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNull__2494() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___149 = temper_std::testing::Test::new();
        let mut t___14740: SafeIdentifier = sid__709("users");
        let mut t___14741: SafeIdentifier = sid__709("email");
        let q__1719: Query = from(t___14740.clone()).where_not_null(t___14741.clone());
        let mut t___14746: bool = Some(q__1719.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email IS NOT NULL");
        #[derive(Clone)]
        struct ClosureGroup___180 {}
        impl ClosureGroup___180 {
            fn fn__14739(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull".to_string());
            }
        }
        let closure_group = ClosureGroup___180 {};
        let fn__14739 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14739())
        };
        test___149.assert(t___14746, fn__14739.clone());
        test___149.soft_fail_to_hard()
    }
    #[test]
    fn whereNullChainedWithWhere__2495() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___150 = temper_std::testing::Test::new();
        let mut t___14726: SafeIdentifier = sid__709("users");
        let mut t___14727: SqlBuilder = SqlBuilder::new();
        t___14727.append_safe("active = ");
        t___14727.append_boolean(true);
        let mut t___14730: SqlFragment = t___14727.accumulated();
        let q__1721: Query = from(t___14726.clone()).r#where(t___14730.clone()).where_null(sid__709("deleted_at"));
        let mut t___14737: bool = Some(q__1721.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___181 {}
        impl ClosureGroup___181 {
            fn fn__14725(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull chained".to_string());
            }
        }
        let closure_group = ClosureGroup___181 {};
        let fn__14725 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14725())
        };
        test___150.assert(t___14737, fn__14725.clone());
        test___150.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNullChainedWithOrWhere__2497() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___151 = temper_std::testing::Test::new();
        let mut t___14712: SafeIdentifier = sid__709("users");
        let mut t___14713: SafeIdentifier = sid__709("deleted_at");
        let mut t___14714: Query = from(t___14712.clone()).where_null(t___14713.clone());
        let mut t___14715: SqlBuilder = SqlBuilder::new();
        t___14715.append_safe("role = ");
        t___14715.append_string("admin");
        let q__1723: Query = t___14714.or_where(t___14715.accumulated());
        let mut t___14723: bool = Some(q__1723.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL OR role = 'admin'");
        #[derive(Clone)]
        struct ClosureGroup___182 {}
        impl ClosureGroup___182 {
            fn fn__14711(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull with orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___182 {};
        let fn__14711 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14711())
        };
        test___151.assert(t___14723, fn__14711.clone());
        test___151.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithIntValues__2499() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___152 = temper_std::testing::Test::new();
        let mut t___14700: SafeIdentifier = sid__709("users");
        let mut t___14701: SafeIdentifier = sid__709("id");
        let mut t___14702: SqlInt32 = SqlInt32::new(1);
        let mut t___14703: SqlInt32 = SqlInt32::new(2);
        let mut t___14704: SqlInt32 = SqlInt32::new(3);
        let q__1725: Query = from(t___14700.clone()).where_in(t___14701.clone(), [SqlPart::new(t___14702.clone()), SqlPart::new(t___14703.clone()), SqlPart::new(t___14704.clone())]);
        let mut t___14709: bool = Some(q__1725.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___183 {}
        impl ClosureGroup___183 {
            fn fn__14699(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn ints".to_string());
            }
        }
        let closure_group = ClosureGroup___183 {};
        let fn__14699 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14699())
        };
        test___152.assert(t___14709, fn__14699.clone());
        test___152.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithStringValuesEscaping__2500() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___153 = temper_std::testing::Test::new();
        let mut t___14689: SafeIdentifier = sid__709("users");
        let mut t___14690: SafeIdentifier = sid__709("name");
        let mut t___14691: SqlString = SqlString::new("Alice");
        let mut t___14692: SqlString = SqlString::new("Bob's");
        let q__1727: Query = from(t___14689.clone()).where_in(t___14690.clone(), [SqlPart::new(t___14691.clone()), SqlPart::new(t___14692.clone())]);
        let mut t___14697: bool = Some(q__1727.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name IN ('Alice', 'Bob''s')");
        #[derive(Clone)]
        struct ClosureGroup___184 {}
        impl ClosureGroup___184 {
            fn fn__14688(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn strings".to_string());
            }
        }
        let closure_group = ClosureGroup___184 {};
        let fn__14688 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14688())
        };
        test___153.assert(t___14697, fn__14688.clone());
        test___153.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithEmptyListProduces1_0__2501() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___154 = temper_std::testing::Test::new();
        let mut t___14680: SafeIdentifier = sid__709("users");
        let mut t___14681: SafeIdentifier = sid__709("id");
        let q__1729: Query = from(t___14680.clone()).where_in(t___14681.clone(), []);
        let mut t___14686: bool = Some(q__1729.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE 1 = 0");
        #[derive(Clone)]
        struct ClosureGroup___185 {}
        impl ClosureGroup___185 {
            fn fn__14679(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn empty".to_string());
            }
        }
        let closure_group = ClosureGroup___185 {};
        let fn__14679 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14679())
        };
        test___154.assert(t___14686, fn__14679.clone());
        test___154.soft_fail_to_hard()
    }
    #[test]
    fn whereInChained__2502() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___155 = temper_std::testing::Test::new();
        let mut t___14664: SafeIdentifier = sid__709("users");
        let mut t___14665: SqlBuilder = SqlBuilder::new();
        t___14665.append_safe("active = ");
        t___14665.append_boolean(true);
        let mut t___14668: SqlFragment = t___14665.accumulated();
        let q__1731: Query = from(t___14664.clone()).r#where(t___14668.clone()).where_in(sid__709("role"), [SqlPart::new(SqlString::new("admin")), SqlPart::new(SqlString::new("user"))]);
        let mut t___14677: bool = Some(q__1731.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND role IN ('admin', 'user')");
        #[derive(Clone)]
        struct ClosureGroup___186 {}
        impl ClosureGroup___186 {
            fn fn__14663(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn chained".to_string());
            }
        }
        let closure_group = ClosureGroup___186 {};
        let fn__14663 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14663())
        };
        test___155.assert(t___14677, fn__14663.clone());
        test___155.soft_fail_to_hard()
    }
    #[test]
    fn whereInSingleElement__2504() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___156 = temper_std::testing::Test::new();
        let mut t___14654: SafeIdentifier = sid__709("users");
        let mut t___14655: SafeIdentifier = sid__709("id");
        let mut t___14656: SqlInt32 = SqlInt32::new(42);
        let q__1733: Query = from(t___14654.clone()).where_in(t___14655.clone(), [SqlPart::new(t___14656.clone())]);
        let mut t___14661: bool = Some(q__1733.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (42)");
        #[derive(Clone)]
        struct ClosureGroup___187 {}
        impl ClosureGroup___187 {
            fn fn__14653(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn single".to_string());
            }
        }
        let closure_group = ClosureGroup___187 {};
        let fn__14653 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14653())
        };
        test___156.assert(t___14661, fn__14653.clone());
        test___156.soft_fail_to_hard()
    }
    #[test]
    fn whereNotBasic__2505() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___157 = temper_std::testing::Test::new();
        let mut t___14642: SafeIdentifier = sid__709("users");
        let mut t___14643: SqlBuilder = SqlBuilder::new();
        t___14643.append_safe("active = ");
        t___14643.append_boolean(true);
        let mut t___14646: SqlFragment = t___14643.accumulated();
        let q__1735: Query = from(t___14642.clone()).where_not(t___14646.clone());
        let mut t___14651: bool = Some(q__1735.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE NOT (active = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___188 {}
        impl ClosureGroup___188 {
            fn fn__14641(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot".to_string());
            }
        }
        let closure_group = ClosureGroup___188 {};
        let fn__14641 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14641())
        };
        test___157.assert(t___14651, fn__14641.clone());
        test___157.soft_fail_to_hard()
    }
    #[test]
    fn whereNotChained__2507() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___158 = temper_std::testing::Test::new();
        let mut t___14625: SafeIdentifier = sid__709("users");
        let mut t___14626: SqlBuilder = SqlBuilder::new();
        t___14626.append_safe("age > ");
        t___14626.append_int32(18);
        let mut t___14629: SqlFragment = t___14626.accumulated();
        let mut t___14630: Query = from(t___14625.clone()).r#where(t___14629.clone());
        let mut t___14631: SqlBuilder = SqlBuilder::new();
        t___14631.append_safe("banned = ");
        t___14631.append_boolean(true);
        let q__1737: Query = t___14630.where_not(t___14631.accumulated());
        let mut t___14639: bool = Some(q__1737.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND NOT (banned = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___189 {}
        impl ClosureGroup___189 {
            fn fn__14624(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot chained".to_string());
            }
        }
        let closure_group = ClosureGroup___189 {};
        let fn__14624 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14624())
        };
        test___158.assert(t___14639, fn__14624.clone());
        test___158.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenIntegers__2510() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___159 = temper_std::testing::Test::new();
        let mut t___14614: SafeIdentifier = sid__709("users");
        let mut t___14615: SafeIdentifier = sid__709("age");
        let mut t___14616: SqlInt32 = SqlInt32::new(18);
        let mut t___14617: SqlInt32 = SqlInt32::new(65);
        let q__1739: Query = from(t___14614.clone()).where_between(t___14615.clone(), SqlPart::new(t___14616.clone()), SqlPart::new(t___14617.clone()));
        let mut t___14622: bool = Some(q__1739.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age BETWEEN 18 AND 65");
        #[derive(Clone)]
        struct ClosureGroup___190 {}
        impl ClosureGroup___190 {
            fn fn__14613(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween ints".to_string());
            }
        }
        let closure_group = ClosureGroup___190 {};
        let fn__14613 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14613())
        };
        test___159.assert(t___14622, fn__14613.clone());
        test___159.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenChained__2511() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___160 = temper_std::testing::Test::new();
        let mut t___14598: SafeIdentifier = sid__709("users");
        let mut t___14599: SqlBuilder = SqlBuilder::new();
        t___14599.append_safe("active = ");
        t___14599.append_boolean(true);
        let mut t___14602: SqlFragment = t___14599.accumulated();
        let q__1741: Query = from(t___14598.clone()).r#where(t___14602.clone()).where_between(sid__709("age"), SqlPart::new(SqlInt32::new(21)), SqlPart::new(SqlInt32::new(30)));
        let mut t___14611: bool = Some(q__1741.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND age BETWEEN 21 AND 30");
        #[derive(Clone)]
        struct ClosureGroup___191 {}
        impl ClosureGroup___191 {
            fn fn__14597(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween chained".to_string());
            }
        }
        let closure_group = ClosureGroup___191 {};
        let fn__14597 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14597())
        };
        test___160.assert(t___14611, fn__14597.clone());
        test___160.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeBasic__2513() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___161 = temper_std::testing::Test::new();
        let mut t___14589: SafeIdentifier = sid__709("users");
        let mut t___14590: SafeIdentifier = sid__709("name");
        let q__1743: Query = from(t___14589.clone()).where_like(t___14590.clone(), "John%");
        let mut t___14595: bool = Some(q__1743.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE 'John%'");
        #[derive(Clone)]
        struct ClosureGroup___192 {}
        impl ClosureGroup___192 {
            fn fn__14588(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike".to_string());
            }
        }
        let closure_group = ClosureGroup___192 {};
        let fn__14588 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14588())
        };
        test___161.assert(t___14595, fn__14588.clone());
        test___161.soft_fail_to_hard()
    }
    #[test]
    fn whereIlikeBasic__2514() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___162 = temper_std::testing::Test::new();
        let mut t___14580: SafeIdentifier = sid__709("users");
        let mut t___14581: SafeIdentifier = sid__709("email");
        let q__1745: Query = from(t___14580.clone()).where_i_like(t___14581.clone(), "%@gmail.com");
        let mut t___14586: bool = Some(q__1745.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email ILIKE '%@gmail.com'");
        #[derive(Clone)]
        struct ClosureGroup___193 {}
        impl ClosureGroup___193 {
            fn fn__14579(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereILike".to_string());
            }
        }
        let closure_group = ClosureGroup___193 {};
        let fn__14579 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14579())
        };
        test___162.assert(t___14586, fn__14579.clone());
        test___162.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWithInjectionAttempt__2515() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___163 = temper_std::testing::Test::new();
        let mut t___14566: SafeIdentifier = sid__709("users");
        let mut t___14567: SafeIdentifier = sid__709("name");
        let q__1747: Query = from(t___14566.clone()).where_like(t___14567.clone(), "'; DROP TABLE users; --");
        let s__1748: std::sync::Arc<String> = q__1747.to_sql().to_string();
        let mut t___14572: bool = temper_core::string::index_of( & s__1748, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___194 {
            s__1748: std::sync::Arc<String>
        }
        impl ClosureGroup___194 {
            fn fn__14565(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like injection escaped: {}", self.s__1748.clone()));
            }
        }
        let closure_group = ClosureGroup___194 {
            s__1748: s__1748.clone()
        };
        let fn__14565 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14565())
        };
        test___163.assert(t___14572, fn__14565.clone());
        let mut t___14576: bool = temper_core::string::index_of( & s__1748, "LIKE", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___195 {
            s__1748: std::sync::Arc<String>
        }
        impl ClosureGroup___195 {
            fn fn__14564(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like structure intact: {}", self.s__1748.clone()));
            }
        }
        let closure_group = ClosureGroup___195 {
            s__1748: s__1748.clone()
        };
        let fn__14564 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14564())
        };
        test___163.assert(t___14576, fn__14564.clone());
        test___163.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWildcardPatterns__2516() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___164 = temper_std::testing::Test::new();
        let mut t___14556: SafeIdentifier = sid__709("users");
        let mut t___14557: SafeIdentifier = sid__709("name");
        let q__1750: Query = from(t___14556.clone()).where_like(t___14557.clone(), "%son%");
        let mut t___14562: bool = Some(q__1750.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE '%son%'");
        #[derive(Clone)]
        struct ClosureGroup___196 {}
        impl ClosureGroup___196 {
            fn fn__14555(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike wildcard".to_string());
            }
        }
        let closure_group = ClosureGroup___196 {};
        let fn__14555 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14555())
        };
        test___164.assert(t___14562, fn__14555.clone());
        test___164.soft_fail_to_hard()
    }
    #[test]
    fn countAllProducesCount__2517() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___165 = temper_std::testing::Test::new();
        let f__1752: SqlFragment = count_all();
        let mut t___14553: bool = Some(f__1752.to_string().as_str()) == Some("COUNT(*)");
        #[derive(Clone)]
        struct ClosureGroup___197 {}
        impl ClosureGroup___197 {
            fn fn__14549(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countAll".to_string());
            }
        }
        let closure_group = ClosureGroup___197 {};
        let fn__14549 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14549())
        };
        test___165.assert(t___14553, fn__14549.clone());
        test___165.soft_fail_to_hard()
    }
    #[test]
    fn countColProducesCountField__2518() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___166 = temper_std::testing::Test::new();
        let f__1754: SqlFragment = count_col(sid__709("id"));
        let mut t___14547: bool = Some(f__1754.to_string().as_str()) == Some("COUNT(id)");
        #[derive(Clone)]
        struct ClosureGroup___198 {}
        impl ClosureGroup___198 {
            fn fn__14542(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countCol".to_string());
            }
        }
        let closure_group = ClosureGroup___198 {};
        let fn__14542 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14542())
        };
        test___166.assert(t___14547, fn__14542.clone());
        test___166.soft_fail_to_hard()
    }
    #[test]
    fn sumColProducesSumField__2519() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___167 = temper_std::testing::Test::new();
        let f__1756: SqlFragment = sum_col(sid__709("amount"));
        let mut t___14540: bool = Some(f__1756.to_string().as_str()) == Some("SUM(amount)");
        #[derive(Clone)]
        struct ClosureGroup___199 {}
        impl ClosureGroup___199 {
            fn fn__14535(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("sumCol".to_string());
            }
        }
        let closure_group = ClosureGroup___199 {};
        let fn__14535 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14535())
        };
        test___167.assert(t___14540, fn__14535.clone());
        test___167.soft_fail_to_hard()
    }
    #[test]
    fn avgColProducesAvgField__2520() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___168 = temper_std::testing::Test::new();
        let f__1758: SqlFragment = avg_col(sid__709("price"));
        let mut t___14533: bool = Some(f__1758.to_string().as_str()) == Some("AVG(price)");
        #[derive(Clone)]
        struct ClosureGroup___200 {}
        impl ClosureGroup___200 {
            fn fn__14528(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("avgCol".to_string());
            }
        }
        let closure_group = ClosureGroup___200 {};
        let fn__14528 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14528())
        };
        test___168.assert(t___14533, fn__14528.clone());
        test___168.soft_fail_to_hard()
    }
    #[test]
    fn minColProducesMinField__2521() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___169 = temper_std::testing::Test::new();
        let f__1760: SqlFragment = min_col(sid__709("created_at"));
        let mut t___14526: bool = Some(f__1760.to_string().as_str()) == Some("MIN(created_at)");
        #[derive(Clone)]
        struct ClosureGroup___201 {}
        impl ClosureGroup___201 {
            fn fn__14521(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("minCol".to_string());
            }
        }
        let closure_group = ClosureGroup___201 {};
        let fn__14521 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14521())
        };
        test___169.assert(t___14526, fn__14521.clone());
        test___169.soft_fail_to_hard()
    }
    #[test]
    fn maxColProducesMaxField__2522() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___170 = temper_std::testing::Test::new();
        let f__1762: SqlFragment = max_col(sid__709("score"));
        let mut t___14519: bool = Some(f__1762.to_string().as_str()) == Some("MAX(score)");
        #[derive(Clone)]
        struct ClosureGroup___202 {}
        impl ClosureGroup___202 {
            fn fn__14514(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("maxCol".to_string());
            }
        }
        let closure_group = ClosureGroup___202 {};
        let fn__14514 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14514())
        };
        test___170.assert(t___14519, fn__14514.clone());
        test___170.soft_fail_to_hard()
    }
    #[test]
    fn selectExprWithAggregate__2523() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___171 = temper_std::testing::Test::new();
        let mut t___14506: SafeIdentifier = sid__709("orders");
        let mut t___14507: SqlFragment = count_all();
        let q__1764: Query = from(t___14506.clone()).select_expr([t___14507.clone()]);
        let mut t___14512: bool = Some(q__1764.to_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM orders");
        #[derive(Clone)]
        struct ClosureGroup___203 {}
        impl ClosureGroup___203 {
            fn fn__14505(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr count".to_string());
            }
        }
        let closure_group = ClosureGroup___203 {};
        let fn__14505 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14505())
        };
        test___171.assert(t___14512, fn__14505.clone());
        test___171.soft_fail_to_hard()
    }
    #[test]
    fn selectExprWithMultipleExpressions__2524() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___172 = temper_std::testing::Test::new();
        let nameFrag__1766: SqlFragment = col(sid__709("users"), sid__709("name"));
        let mut t___14497: SafeIdentifier = sid__709("users");
        let mut t___14498: SqlFragment = count_all();
        let q__1767: Query = from(t___14497.clone()).select_expr([nameFrag__1766.clone(), t___14498.clone()]);
        let mut t___14503: bool = Some(q__1767.to_sql().to_string().as_str()) == Some("SELECT users.name, COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___204 {}
        impl ClosureGroup___204 {
            fn fn__14493(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr multi".to_string());
            }
        }
        let closure_group = ClosureGroup___204 {};
        let fn__14493 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14493())
        };
        test___172.assert(t___14503, fn__14493.clone());
        test___172.soft_fail_to_hard()
    }
    #[test]
    fn selectExprOverridesSelectedFields__2525() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___173 = temper_std::testing::Test::new();
        let mut t___14482: SafeIdentifier = sid__709("users");
        let mut t___14483: SafeIdentifier = sid__709("id");
        let mut t___14484: SafeIdentifier = sid__709("name");
        let q__1769: Query = from(t___14482.clone()).select([t___14483.clone(), t___14484.clone()]).select_expr([count_all()]);
        let mut t___14491: bool = Some(q__1769.to_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___205 {}
        impl ClosureGroup___205 {
            fn fn__14481(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr overrides select".to_string());
            }
        }
        let closure_group = ClosureGroup___205 {};
        let fn__14481 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14481())
        };
        test___173.assert(t___14491, fn__14481.clone());
        test___173.soft_fail_to_hard()
    }
    #[test]
    fn groupBySingleField__2526() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___174 = temper_std::testing::Test::new();
        let mut t___14468: SafeIdentifier = sid__709("orders");
        let mut t___14471: SqlFragment = col(sid__709("orders"), sid__709("status"));
        let mut t___14472: SqlFragment = count_all();
        let q__1771: Query = from(t___14468.clone()).select_expr([t___14471.clone(), t___14472.clone()]).group_by(sid__709("status"));
        let mut t___14479: bool = Some(q__1771.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*) FROM orders GROUP BY status");
        #[derive(Clone)]
        struct ClosureGroup___206 {}
        impl ClosureGroup___206 {
            fn fn__14467(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("groupBy single".to_string());
            }
        }
        let closure_group = ClosureGroup___206 {};
        let fn__14467 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14467())
        };
        test___174.assert(t___14479, fn__14467.clone());
        test___174.soft_fail_to_hard()
    }
    #[test]
    fn groupByMultipleFields__2527() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___175 = temper_std::testing::Test::new();
        let mut t___14457: SafeIdentifier = sid__709("orders");
        let mut t___14458: SafeIdentifier = sid__709("status");
        let q__1773: Query = from(t___14457.clone()).group_by(t___14458.clone()).group_by(sid__709("category"));
        let mut t___14465: bool = Some(q__1773.to_sql().to_string().as_str()) == Some("SELECT * FROM orders GROUP BY status, category");
        #[derive(Clone)]
        struct ClosureGroup___207 {}
        impl ClosureGroup___207 {
            fn fn__14456(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("groupBy multiple".to_string());
            }
        }
        let closure_group = ClosureGroup___207 {};
        let fn__14456 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14456())
        };
        test___175.assert(t___14465, fn__14456.clone());
        test___175.soft_fail_to_hard()
    }
    #[test]
    fn havingBasic__2528() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___176 = temper_std::testing::Test::new();
        let mut t___14438: SafeIdentifier = sid__709("orders");
        let mut t___14441: SqlFragment = col(sid__709("orders"), sid__709("status"));
        let mut t___14442: SqlFragment = count_all();
        let mut t___14445: Query = from(t___14438.clone()).select_expr([t___14441.clone(), t___14442.clone()]).group_by(sid__709("status"));
        let mut t___14446: SqlBuilder = SqlBuilder::new();
        t___14446.append_safe("COUNT(*) > ");
        t___14446.append_int32(5);
        let q__1775: Query = t___14445.having(t___14446.accumulated());
        let mut t___14454: bool = Some(q__1775.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*) FROM orders GROUP BY status HAVING COUNT(*) > 5");
        #[derive(Clone)]
        struct ClosureGroup___208 {}
        impl ClosureGroup___208 {
            fn fn__14437(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("having basic".to_string());
            }
        }
        let closure_group = ClosureGroup___208 {};
        let fn__14437 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14437())
        };
        test___176.assert(t___14454, fn__14437.clone());
        test___176.soft_fail_to_hard()
    }
    #[test]
    fn orHaving__2530() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___177 = temper_std::testing::Test::new();
        let mut t___14419: SafeIdentifier = sid__709("orders");
        let mut t___14420: SafeIdentifier = sid__709("status");
        let mut t___14421: Query = from(t___14419.clone()).group_by(t___14420.clone());
        let mut t___14422: SqlBuilder = SqlBuilder::new();
        t___14422.append_safe("COUNT(*) > ");
        t___14422.append_int32(5);
        let mut t___14426: Query = t___14421.having(t___14422.accumulated());
        let mut t___14427: SqlBuilder = SqlBuilder::new();
        t___14427.append_safe("SUM(total) > ");
        t___14427.append_int32(1000);
        let q__1777: Query = t___14426.or_having(t___14427.accumulated());
        let mut t___14435: bool = Some(q__1777.to_sql().to_string().as_str()) == Some("SELECT * FROM orders GROUP BY status HAVING COUNT(*) > 5 OR SUM(total) > 1000");
        #[derive(Clone)]
        struct ClosureGroup___209 {}
        impl ClosureGroup___209 {
            fn fn__14418(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orHaving".to_string());
            }
        }
        let closure_group = ClosureGroup___209 {};
        let fn__14418 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14418())
        };
        test___177.assert(t___14435, fn__14418.clone());
        test___177.soft_fail_to_hard()
    }
    #[test]
    fn distinctBasic__2533() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___178 = temper_std::testing::Test::new();
        let mut t___14409: SafeIdentifier = sid__709("users");
        let mut t___14410: SafeIdentifier = sid__709("name");
        let q__1779: Query = from(t___14409.clone()).select([t___14410.clone()]).distinct();
        let mut t___14416: bool = Some(q__1779.to_sql().to_string().as_str()) == Some("SELECT DISTINCT name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___210 {}
        impl ClosureGroup___210 {
            fn fn__14408(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("distinct".to_string());
            }
        }
        let closure_group = ClosureGroup___210 {};
        let fn__14408 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14408())
        };
        test___178.assert(t___14416, fn__14408.clone());
        test___178.soft_fail_to_hard()
    }
    #[test]
    fn distinctWithWhere__2534() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___179 = temper_std::testing::Test::new();
        let mut t___14394: SafeIdentifier = sid__709("users");
        let mut t___14395: SafeIdentifier = sid__709("email");
        let mut t___14396: Query = from(t___14394.clone()).select([t___14395.clone()]);
        let mut t___14397: SqlBuilder = SqlBuilder::new();
        t___14397.append_safe("active = ");
        t___14397.append_boolean(true);
        let q__1781: Query = t___14396.r#where(t___14397.accumulated()).distinct();
        let mut t___14406: bool = Some(q__1781.to_sql().to_string().as_str()) == Some("SELECT DISTINCT email FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___211 {}
        impl ClosureGroup___211 {
            fn fn__14393(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("distinct with where".to_string());
            }
        }
        let closure_group = ClosureGroup___211 {};
        let fn__14393 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14393())
        };
        test___179.assert(t___14406, fn__14393.clone());
        test___179.soft_fail_to_hard()
    }
    #[test]
    fn countSqlBare__2536() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___180 = temper_std::testing::Test::new();
        let q__1783: Query = from(sid__709("users"));
        let mut t___14391: bool = Some(q__1783.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___212 {}
        impl ClosureGroup___212 {
            fn fn__14386(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql bare".to_string());
            }
        }
        let closure_group = ClosureGroup___212 {};
        let fn__14386 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14386())
        };
        test___180.assert(t___14391, fn__14386.clone());
        test___180.soft_fail_to_hard()
    }
    #[test]
    fn countSqlWithWhere__2537() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___181 = temper_std::testing::Test::new();
        let mut t___14375: SafeIdentifier = sid__709("users");
        let mut t___14376: SqlBuilder = SqlBuilder::new();
        t___14376.append_safe("active = ");
        t___14376.append_boolean(true);
        let mut t___14379: SqlFragment = t___14376.accumulated();
        let q__1785: Query = from(t___14375.clone()).r#where(t___14379.clone());
        let mut t___14384: bool = Some(q__1785.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___213 {}
        impl ClosureGroup___213 {
            fn fn__14374(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql with where".to_string());
            }
        }
        let closure_group = ClosureGroup___213 {};
        let fn__14374 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14374())
        };
        test___181.assert(t___14384, fn__14374.clone());
        test___181.soft_fail_to_hard()
    }
    #[test]
    fn countSqlWithJoin__2539() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___182 = temper_std::testing::Test::new();
        let mut t___14358: SafeIdentifier = sid__709("users");
        let mut t___14359: SafeIdentifier = sid__709("orders");
        let mut t___14360: SqlBuilder = SqlBuilder::new();
        t___14360.append_safe("users.id = orders.user_id");
        let mut t___14362: SqlFragment = t___14360.accumulated();
        let mut t___14363: Query = from(t___14358.clone()).inner_join(t___14359.clone(), t___14362.clone());
        let mut t___14364: SqlBuilder = SqlBuilder::new();
        t___14364.append_safe("orders.total > ");
        t___14364.append_int32(100);
        let q__1787: Query = t___14363.r#where(t___14364.accumulated());
        let mut t___14372: bool = Some(q__1787.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100");
        #[derive(Clone)]
        struct ClosureGroup___214 {}
        impl ClosureGroup___214 {
            fn fn__14357(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql with join".to_string());
            }
        }
        let closure_group = ClosureGroup___214 {};
        let fn__14357 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14357())
        };
        test___182.assert(t___14372, fn__14357.clone());
        test___182.soft_fail_to_hard()
    }
    #[test]
    fn countSqlDropsOrderByLimitOffset__2542() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___183 = temper_std::testing::Test::new();
        let mut t___14344: SafeIdentifier;
        let mut t___14345: SqlBuilder;
        let mut t___14348: SqlFragment;
        let mut t___7123: Query;
        let mut t___7124: Query;
        let q__1789: Query;
        'ok___17456: {
            'orelse___2773: {
                t___14344 = sid__709("users");
                t___14345 = SqlBuilder::new();
                t___14345.append_safe("active = ");
                t___14345.append_boolean(true);
                t___14348 = t___14345.accumulated();
                t___7123 = match from(t___14344.clone()).r#where(t___14348.clone()).order_by(sid__709("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___2773
                };
                t___7124 = match t___7123.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___2773
                };
                q__1789 = t___7124.clone();
                break 'ok___17456;
            }
            q__1789 = panic!();
        }
        let s__1790: std::sync::Arc<String> = q__1789.count_sql().to_string();
        let mut t___14355: bool = Some(s__1790.as_str()) == Some("SELECT COUNT(*) FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___215 {
            s__1790: std::sync::Arc<String>
        }
        impl ClosureGroup___215 {
            fn fn__14343(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("countSql drops extras: {}", self.s__1790.clone()));
            }
        }
        let closure_group = ClosureGroup___215 {
            s__1790: s__1790.clone()
        };
        let fn__14343 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14343())
        };
        test___183.assert(t___14355, fn__14343.clone());
        test___183.soft_fail_to_hard()
    }
    #[test]
    fn fullAggregationQuery__2544() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___184 = temper_std::testing::Test::new();
        let mut t___14311: SafeIdentifier = sid__709("orders");
        let mut t___14314: SqlFragment = col(sid__709("orders"), sid__709("status"));
        let mut t___14315: SqlFragment = count_all();
        let mut t___14317: SqlFragment = sum_col(sid__709("total"));
        let mut t___14318: Query = from(t___14311.clone()).select_expr([t___14314.clone(), t___14315.clone(), t___14317.clone()]);
        let mut t___14319: SafeIdentifier = sid__709("users");
        let mut t___14320: SqlBuilder = SqlBuilder::new();
        t___14320.append_safe("orders.user_id = users.id");
        let mut t___14323: Query = t___14318.inner_join(t___14319.clone(), t___14320.accumulated());
        let mut t___14324: SqlBuilder = SqlBuilder::new();
        t___14324.append_safe("users.active = ");
        t___14324.append_boolean(true);
        let mut t___14330: Query = t___14323.r#where(t___14324.accumulated()).group_by(sid__709("status"));
        let mut t___14331: SqlBuilder = SqlBuilder::new();
        t___14331.append_safe("COUNT(*) > ");
        t___14331.append_int32(3);
        let q__1792: Query = t___14330.having(t___14331.accumulated()).order_by(sid__709("status"), true);
        let expected__1793: std::sync::Arc<String> = std::sync::Arc::new("SELECT orders.status, COUNT(*), SUM(total) FROM orders INNER JOIN users ON orders.user_id = users.id WHERE users.active = TRUE GROUP BY status HAVING COUNT(*) > 3 ORDER BY status ASC".to_string());
        let mut t___14341: bool = Some(q__1792.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*), SUM(total) FROM orders INNER JOIN users ON orders.user_id = users.id WHERE users.active = TRUE GROUP BY status HAVING COUNT(*) > 3 ORDER BY status ASC");
        #[derive(Clone)]
        struct ClosureGroup___216 {}
        impl ClosureGroup___216 {
            fn fn__14310(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full aggregation".to_string());
            }
        }
        let closure_group = ClosureGroup___216 {};
        let fn__14310 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14310())
        };
        test___184.assert(t___14341, fn__14310.clone());
        test___184.soft_fail_to_hard()
    }
    #[test]
    fn unionSql__2548() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___185 = temper_std::testing::Test::new();
        let mut t___14293: SafeIdentifier = sid__709("users");
        let mut t___14294: SqlBuilder = SqlBuilder::new();
        t___14294.append_safe("role = ");
        t___14294.append_string("admin");
        let mut t___14297: SqlFragment = t___14294.accumulated();
        let a__1795: Query = from(t___14293.clone()).r#where(t___14297.clone());
        let mut t___14299: SafeIdentifier = sid__709("users");
        let mut t___14300: SqlBuilder = SqlBuilder::new();
        t___14300.append_safe("role = ");
        t___14300.append_string("moderator");
        let mut t___14303: SqlFragment = t___14300.accumulated();
        let b__1796: Query = from(t___14299.clone()).r#where(t___14303.clone());
        let s__1797: std::sync::Arc<String> = union_sql(a__1795.clone(), b__1796.clone()).to_string();
        let mut t___14308: bool = Some(s__1797.as_str()) == Some("(SELECT * FROM users WHERE role = 'admin') UNION (SELECT * FROM users WHERE role = 'moderator')");
        #[derive(Clone)]
        struct ClosureGroup___217 {
            s__1797: std::sync::Arc<String>
        }
        impl ClosureGroup___217 {
            fn fn__14292(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("unionSql: {}", self.s__1797.clone()));
            }
        }
        let closure_group = ClosureGroup___217 {
            s__1797: s__1797.clone()
        };
        let fn__14292 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14292())
        };
        test___185.assert(t___14308, fn__14292.clone());
        test___185.soft_fail_to_hard()
    }
    #[test]
    fn unionAllSql__2551() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___186 = temper_std::testing::Test::new();
        let mut t___14281: SafeIdentifier = sid__709("users");
        let mut t___14282: SafeIdentifier = sid__709("name");
        let a__1799: Query = from(t___14281.clone()).select([t___14282.clone()]);
        let mut t___14284: SafeIdentifier = sid__709("contacts");
        let mut t___14285: SafeIdentifier = sid__709("name");
        let b__1800: Query = from(t___14284.clone()).select([t___14285.clone()]);
        let s__1801: std::sync::Arc<String> = union_all_sql(a__1799.clone(), b__1800.clone()).to_string();
        let mut t___14290: bool = Some(s__1801.as_str()) == Some("(SELECT name FROM users) UNION ALL (SELECT name FROM contacts)");
        #[derive(Clone)]
        struct ClosureGroup___218 {
            s__1801: std::sync::Arc<String>
        }
        impl ClosureGroup___218 {
            fn fn__14280(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("unionAllSql: {}", self.s__1801.clone()));
            }
        }
        let closure_group = ClosureGroup___218 {
            s__1801: s__1801.clone()
        };
        let fn__14280 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14280())
        };
        test___186.assert(t___14290, fn__14280.clone());
        test___186.soft_fail_to_hard()
    }
    #[test]
    fn intersectSql__2552() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___187 = temper_std::testing::Test::new();
        let mut t___14269: SafeIdentifier = sid__709("users");
        let mut t___14270: SafeIdentifier = sid__709("email");
        let a__1803: Query = from(t___14269.clone()).select([t___14270.clone()]);
        let mut t___14272: SafeIdentifier = sid__709("subscribers");
        let mut t___14273: SafeIdentifier = sid__709("email");
        let b__1804: Query = from(t___14272.clone()).select([t___14273.clone()]);
        let s__1805: std::sync::Arc<String> = intersect_sql(a__1803.clone(), b__1804.clone()).to_string();
        let mut t___14278: bool = Some(s__1805.as_str()) == Some("(SELECT email FROM users) INTERSECT (SELECT email FROM subscribers)");
        #[derive(Clone)]
        struct ClosureGroup___219 {
            s__1805: std::sync::Arc<String>
        }
        impl ClosureGroup___219 {
            fn fn__14268(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("intersectSql: {}", self.s__1805.clone()));
            }
        }
        let closure_group = ClosureGroup___219 {
            s__1805: s__1805.clone()
        };
        let fn__14268 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14268())
        };
        test___187.assert(t___14278, fn__14268.clone());
        test___187.soft_fail_to_hard()
    }
    #[test]
    fn exceptSql__2553() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___188 = temper_std::testing::Test::new();
        let mut t___14257: SafeIdentifier = sid__709("users");
        let mut t___14258: SafeIdentifier = sid__709("id");
        let a__1807: Query = from(t___14257.clone()).select([t___14258.clone()]);
        let mut t___14260: SafeIdentifier = sid__709("banned");
        let mut t___14261: SafeIdentifier = sid__709("id");
        let b__1808: Query = from(t___14260.clone()).select([t___14261.clone()]);
        let s__1809: std::sync::Arc<String> = except_sql(a__1807.clone(), b__1808.clone()).to_string();
        let mut t___14266: bool = Some(s__1809.as_str()) == Some("(SELECT id FROM users) EXCEPT (SELECT id FROM banned)");
        #[derive(Clone)]
        struct ClosureGroup___220 {
            s__1809: std::sync::Arc<String>
        }
        impl ClosureGroup___220 {
            fn fn__14256(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("exceptSql: {}", self.s__1809.clone()));
            }
        }
        let closure_group = ClosureGroup___220 {
            s__1809: s__1809.clone()
        };
        let fn__14256 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14256())
        };
        test___188.assert(t___14266, fn__14256.clone());
        test___188.soft_fail_to_hard()
    }
    #[test]
    fn subqueryWithAlias__2554() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___189 = temper_std::testing::Test::new();
        let mut t___14242: SafeIdentifier = sid__709("orders");
        let mut t___14243: SafeIdentifier = sid__709("user_id");
        let mut t___14244: Query = from(t___14242.clone()).select([t___14243.clone()]);
        let mut t___14245: SqlBuilder = SqlBuilder::new();
        t___14245.append_safe("total > ");
        t___14245.append_int32(100);
        let inner__1811: Query = t___14244.r#where(t___14245.accumulated());
        let s__1812: std::sync::Arc<String> = subquery(inner__1811.clone(), sid__709("big_orders")).to_string();
        let mut t___14254: bool = Some(s__1812.as_str()) == Some("(SELECT user_id FROM orders WHERE total > 100) AS big_orders");
        #[derive(Clone)]
        struct ClosureGroup___221 {
            s__1812: std::sync::Arc<String>
        }
        impl ClosureGroup___221 {
            fn fn__14241(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("subquery: {}", self.s__1812.clone()));
            }
        }
        let closure_group = ClosureGroup___221 {
            s__1812: s__1812.clone()
        };
        let fn__14241 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14241())
        };
        test___189.assert(t___14254, fn__14241.clone());
        test___189.soft_fail_to_hard()
    }
    #[test]
    fn existsSql__2556() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___190 = temper_std::testing::Test::new();
        let mut t___14231: SafeIdentifier = sid__709("orders");
        let mut t___14232: SqlBuilder = SqlBuilder::new();
        t___14232.append_safe("orders.user_id = users.id");
        let mut t___14234: SqlFragment = t___14232.accumulated();
        let inner__1814: Query = from(t___14231.clone()).r#where(t___14234.clone());
        let s__1815: std::sync::Arc<String> = exists_sql(inner__1814.clone()).to_string();
        let mut t___14239: bool = Some(s__1815.as_str()) == Some("EXISTS (SELECT * FROM orders WHERE orders.user_id = users.id)");
        #[derive(Clone)]
        struct ClosureGroup___222 {
            s__1815: std::sync::Arc<String>
        }
        impl ClosureGroup___222 {
            fn fn__14230(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("existsSql: {}", self.s__1815.clone()));
            }
        }
        let closure_group = ClosureGroup___222 {
            s__1815: s__1815.clone()
        };
        let fn__14230 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14230())
        };
        test___190.assert(t___14239, fn__14230.clone());
        test___190.soft_fail_to_hard()
    }
    #[test]
    fn whereInSubquery__2558() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___191 = temper_std::testing::Test::new();
        let mut t___14214: SafeIdentifier = sid__709("orders");
        let mut t___14215: SafeIdentifier = sid__709("user_id");
        let mut t___14216: Query = from(t___14214.clone()).select([t___14215.clone()]);
        let mut t___14217: SqlBuilder = SqlBuilder::new();
        t___14217.append_safe("total > ");
        t___14217.append_int32(1000);
        let sub__1817: Query = t___14216.r#where(t___14217.accumulated());
        let mut t___14222: SafeIdentifier = sid__709("users");
        let mut t___14223: SafeIdentifier = sid__709("id");
        let q__1818: Query = from(t___14222.clone()).where_in_subquery(t___14223.clone(), sub__1817.clone());
        let s__1819: std::sync::Arc<String> = q__1818.to_sql().to_string();
        let mut t___14228: bool = Some(s__1819.as_str()) == Some("SELECT * FROM users WHERE id IN (SELECT user_id FROM orders WHERE total > 1000)");
        #[derive(Clone)]
        struct ClosureGroup___223 {
            s__1819: std::sync::Arc<String>
        }
        impl ClosureGroup___223 {
            fn fn__14213(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("whereInSubquery: {}", self.s__1819.clone()));
            }
        }
        let closure_group = ClosureGroup___223 {
            s__1819: s__1819.clone()
        };
        let fn__14213 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14213())
        };
        test___191.assert(t___14228, fn__14213.clone());
        test___191.soft_fail_to_hard()
    }
    #[test]
    fn setOperationWithWhereOnEachSide__2560() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___192 = temper_std::testing::Test::new();
        let mut t___14191: SafeIdentifier = sid__709("users");
        let mut t___14192: SqlBuilder = SqlBuilder::new();
        t___14192.append_safe("age > ");
        t___14192.append_int32(18);
        let mut t___14195: SqlFragment = t___14192.accumulated();
        let mut t___14196: Query = from(t___14191.clone()).r#where(t___14195.clone());
        let mut t___14197: SqlBuilder = SqlBuilder::new();
        t___14197.append_safe("active = ");
        t___14197.append_boolean(true);
        let a__1821: Query = t___14196.r#where(t___14197.accumulated());
        let mut t___14202: SafeIdentifier = sid__709("users");
        let mut t___14203: SqlBuilder = SqlBuilder::new();
        t___14203.append_safe("role = ");
        t___14203.append_string("vip");
        let mut t___14206: SqlFragment = t___14203.accumulated();
        let b__1822: Query = from(t___14202.clone()).r#where(t___14206.clone());
        let s__1823: std::sync::Arc<String> = union_sql(a__1821.clone(), b__1822.clone()).to_string();
        let mut t___14211: bool = Some(s__1823.as_str()) == Some("(SELECT * FROM users WHERE age > 18 AND active = TRUE) UNION (SELECT * FROM users WHERE role = 'vip')");
        #[derive(Clone)]
        struct ClosureGroup___224 {
            s__1823: std::sync::Arc<String>
        }
        impl ClosureGroup___224 {
            fn fn__14190(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("union with where: {}", self.s__1823.clone()));
            }
        }
        let closure_group = ClosureGroup___224 {
            s__1823: s__1823.clone()
        };
        let fn__14190 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14190())
        };
        test___192.assert(t___14211, fn__14190.clone());
        test___192.soft_fail_to_hard()
    }
    #[test]
    fn whereInSubqueryChainedWithWhere__2564() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___193 = temper_std::testing::Test::new();
        let mut t___14174: SafeIdentifier = sid__709("orders");
        let mut t___14175: SafeIdentifier = sid__709("user_id");
        let sub__1825: Query = from(t___14174.clone()).select([t___14175.clone()]);
        let mut t___14177: SafeIdentifier = sid__709("users");
        let mut t___14178: SqlBuilder = SqlBuilder::new();
        t___14178.append_safe("active = ");
        t___14178.append_boolean(true);
        let mut t___14181: SqlFragment = t___14178.accumulated();
        let q__1826: Query = from(t___14177.clone()).r#where(t___14181.clone()).where_in_subquery(sid__709("id"), sub__1825.clone());
        let s__1827: std::sync::Arc<String> = q__1826.to_sql().to_string();
        let mut t___14188: bool = Some(s__1827.as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND id IN (SELECT user_id FROM orders)");
        #[derive(Clone)]
        struct ClosureGroup___225 {
            s__1827: std::sync::Arc<String>
        }
        impl ClosureGroup___225 {
            fn fn__14173(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("whereInSubquery chained: {}", self.s__1827.clone()));
            }
        }
        let closure_group = ClosureGroup___225 {
            s__1827: s__1827.clone()
        };
        let fn__14173 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14173())
        };
        test___193.assert(t___14188, fn__14173.clone());
        test___193.soft_fail_to_hard()
    }
    #[test]
    fn existsSqlUsedInWhere__2566() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___194 = temper_std::testing::Test::new();
        let mut t___14160: SafeIdentifier = sid__709("orders");
        let mut t___14161: SqlBuilder = SqlBuilder::new();
        t___14161.append_safe("orders.user_id = users.id");
        let mut t___14163: SqlFragment = t___14161.accumulated();
        let sub__1829: Query = from(t___14160.clone()).r#where(t___14163.clone());
        let mut t___14165: SafeIdentifier = sid__709("users");
        let mut t___14166: SqlFragment = exists_sql(sub__1829.clone());
        let q__1830: Query = from(t___14165.clone()).r#where(t___14166.clone());
        let s__1831: std::sync::Arc<String> = q__1830.to_sql().to_string();
        let mut t___14171: bool = Some(s__1831.as_str()) == Some("SELECT * FROM users WHERE EXISTS (SELECT * FROM orders WHERE orders.user_id = users.id)");
        #[derive(Clone)]
        struct ClosureGroup___226 {
            s__1831: std::sync::Arc<String>
        }
        impl ClosureGroup___226 {
            fn fn__14159(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("exists in where: {}", self.s__1831.clone()));
            }
        }
        let closure_group = ClosureGroup___226 {
            s__1831: s__1831.clone()
        };
        let fn__14159 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14159())
        };
        test___194.assert(t___14171, fn__14159.clone());
        test___194.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBasic__2568() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___195 = temper_std::testing::Test::new();
        let mut t___14146: SafeIdentifier;
        let mut t___14147: SafeIdentifier;
        let mut t___14148: SqlString;
        let mut t___14149: UpdateQuery;
        let mut t___14150: SqlBuilder;
        let mut t___6945: SqlFragment;
        let q__1833: SqlFragment;
        'ok___17457: {
            'orelse___2774: {
                t___14146 = sid__709("users");
                t___14147 = sid__709("name");
                t___14148 = SqlString::new("Alice");
                t___14149 = update(t___14146.clone()).set(t___14147.clone(), SqlPart::new(t___14148.clone()));
                t___14150 = SqlBuilder::new();
                t___14150.append_safe("id = ");
                t___14150.append_int32(1);
                t___6945 = match t___14149.r#where(t___14150.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2774
                };
                q__1833 = t___6945.clone();
                break 'ok___17457;
            }
            q__1833 = panic!();
        }
        let mut t___14157: bool = Some(q__1833.to_string().as_str()) == Some("UPDATE users SET name = 'Alice' WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___227 {}
        impl ClosureGroup___227 {
            fn fn__14145(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update basic".to_string());
            }
        }
        let closure_group = ClosureGroup___227 {};
        let fn__14145 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14145())
        };
        test___195.assert(t___14157, fn__14145.clone());
        test___195.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryMultipleSet__2570() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___196 = temper_std::testing::Test::new();
        let mut t___14129: SafeIdentifier;
        let mut t___14130: SafeIdentifier;
        let mut t___14131: SqlString;
        let mut t___14135: UpdateQuery;
        let mut t___14136: SqlBuilder;
        let mut t___6930: SqlFragment;
        let q__1835: SqlFragment;
        'ok___17458: {
            'orelse___2775: {
                t___14129 = sid__709("users");
                t___14130 = sid__709("name");
                t___14131 = SqlString::new("Bob");
                t___14135 = update(t___14129.clone()).set(t___14130.clone(), SqlPart::new(t___14131.clone())).set(sid__709("age"), SqlPart::new(SqlInt32::new(30)));
                t___14136 = SqlBuilder::new();
                t___14136.append_safe("id = ");
                t___14136.append_int32(2);
                t___6930 = match t___14135.r#where(t___14136.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2775
                };
                q__1835 = t___6930.clone();
                break 'ok___17458;
            }
            q__1835 = panic!();
        }
        let mut t___14143: bool = Some(q__1835.to_string().as_str()) == Some("UPDATE users SET name = 'Bob', age = 30 WHERE id = 2");
        #[derive(Clone)]
        struct ClosureGroup___228 {}
        impl ClosureGroup___228 {
            fn fn__14128(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update multi set".to_string());
            }
        }
        let closure_group = ClosureGroup___228 {};
        let fn__14128 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14128())
        };
        test___196.assert(t___14143, fn__14128.clone());
        test___196.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryMultipleWhere__2572() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___197 = temper_std::testing::Test::new();
        let mut t___14110: SafeIdentifier;
        let mut t___14111: SafeIdentifier;
        let mut t___14112: SqlBoolean;
        let mut t___14113: UpdateQuery;
        let mut t___14114: SqlBuilder;
        let mut t___14118: UpdateQuery;
        let mut t___14119: SqlBuilder;
        let mut t___6912: SqlFragment;
        let q__1837: SqlFragment;
        'ok___17459: {
            'orelse___2776: {
                t___14110 = sid__709("users");
                t___14111 = sid__709("active");
                t___14112 = SqlBoolean::new(false);
                t___14113 = update(t___14110.clone()).set(t___14111.clone(), SqlPart::new(t___14112.clone()));
                t___14114 = SqlBuilder::new();
                t___14114.append_safe("age < ");
                t___14114.append_int32(18);
                t___14118 = t___14113.r#where(t___14114.accumulated());
                t___14119 = SqlBuilder::new();
                t___14119.append_safe("role = ");
                t___14119.append_string("guest");
                t___6912 = match t___14118.r#where(t___14119.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2776
                };
                q__1837 = t___6912.clone();
                break 'ok___17459;
            }
            q__1837 = panic!();
        }
        let mut t___14126: bool = Some(q__1837.to_string().as_str()) == Some("UPDATE users SET active = FALSE WHERE age < 18 AND role = 'guest'");
        #[derive(Clone)]
        struct ClosureGroup___229 {}
        impl ClosureGroup___229 {
            fn fn__14109(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update multi where".to_string());
            }
        }
        let closure_group = ClosureGroup___229 {};
        let fn__14109 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14109())
        };
        test___197.assert(t___14126, fn__14109.clone());
        test___197.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryOrWhere__2575() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___198 = temper_std::testing::Test::new();
        let mut t___14091: SafeIdentifier;
        let mut t___14092: SafeIdentifier;
        let mut t___14093: SqlString;
        let mut t___14094: UpdateQuery;
        let mut t___14095: SqlBuilder;
        let mut t___14099: UpdateQuery;
        let mut t___14100: SqlBuilder;
        let mut t___6891: SqlFragment;
        let q__1839: SqlFragment;
        'ok___17460: {
            'orelse___2777: {
                t___14091 = sid__709("users");
                t___14092 = sid__709("status");
                t___14093 = SqlString::new("banned");
                t___14094 = update(t___14091.clone()).set(t___14092.clone(), SqlPart::new(t___14093.clone()));
                t___14095 = SqlBuilder::new();
                t___14095.append_safe("spam_count > ");
                t___14095.append_int32(10);
                t___14099 = t___14094.r#where(t___14095.accumulated());
                t___14100 = SqlBuilder::new();
                t___14100.append_safe("reported = ");
                t___14100.append_boolean(true);
                t___6891 = match t___14099.or_where(t___14100.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2777
                };
                q__1839 = t___6891.clone();
                break 'ok___17460;
            }
            q__1839 = panic!();
        }
        let mut t___14107: bool = Some(q__1839.to_string().as_str()) == Some("UPDATE users SET status = 'banned' WHERE spam_count > 10 OR reported = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___230 {}
        impl ClosureGroup___230 {
            fn fn__14090(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___230 {};
        let fn__14090 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14090())
        };
        test___198.assert(t___14107, fn__14090.clone());
        test___198.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBubblesWithoutWhere__2578() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___199 = temper_std::testing::Test::new();
        let mut t___14084: SafeIdentifier;
        let mut t___14085: SafeIdentifier;
        let mut t___14086: SqlInt32;
        let didBubble__1841: bool;
        'ok___17461: {
            'orelse___2778: {
                t___14084 = sid__709("users");
                t___14085 = sid__709("x");
                t___14086 = SqlInt32::new(1);
                match update(t___14084.clone()).set(t___14085.clone(), SqlPart::new(t___14086.clone())).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2778
                };
                didBubble__1841 = false;
                break 'ok___17461;
            }
            didBubble__1841 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___231 {}
        impl ClosureGroup___231 {
            fn fn__14083(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update without WHERE should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___231 {};
        let fn__14083 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14083())
        };
        test___199.assert(didBubble__1841, fn__14083.clone());
        test___199.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBubblesWithoutSet__2579() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___200 = temper_std::testing::Test::new();
        let mut t___14075: SafeIdentifier;
        let mut t___14076: SqlBuilder;
        let mut t___14079: SqlFragment;
        let didBubble__1843: bool;
        'ok___17462: {
            'orelse___2779: {
                t___14075 = sid__709("users");
                t___14076 = SqlBuilder::new();
                t___14076.append_safe("id = ");
                t___14076.append_int32(1);
                t___14079 = t___14076.accumulated();
                match update(t___14075.clone()).r#where(t___14079.clone()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2779
                };
                didBubble__1843 = false;
                break 'ok___17462;
            }
            didBubble__1843 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___232 {}
        impl ClosureGroup___232 {
            fn fn__14074(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update without SET should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___232 {};
        let fn__14074 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14074())
        };
        test___200.assert(didBubble__1843, fn__14074.clone());
        test___200.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryWithLimit__2581() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___201 = temper_std::testing::Test::new();
        let mut t___14061: SafeIdentifier;
        let mut t___14062: SafeIdentifier;
        let mut t___14063: SqlBoolean;
        let mut t___14064: UpdateQuery;
        let mut t___14065: SqlBuilder;
        let mut t___6854: UpdateQuery;
        let mut t___6855: SqlFragment;
        let q__1845: SqlFragment;
        'ok___17463: {
            'orelse___2780: {
                t___14061 = sid__709("users");
                t___14062 = sid__709("active");
                t___14063 = SqlBoolean::new(false);
                t___14064 = update(t___14061.clone()).set(t___14062.clone(), SqlPart::new(t___14063.clone()));
                t___14065 = SqlBuilder::new();
                t___14065.append_safe("last_login < ");
                t___14065.append_string("2024-01-01");
                t___6854 = match t___14064.r#where(t___14065.accumulated()).limit(100) {
                    Ok(x) => x,
                    _ => break 'orelse___2780
                };
                t___6855 = match t___6854.to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2780
                };
                q__1845 = t___6855.clone();
                break 'ok___17463;
            }
            q__1845 = panic!();
        }
        let mut t___14072: bool = Some(q__1845.to_string().as_str()) == Some("UPDATE users SET active = FALSE WHERE last_login < '2024-01-01' LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___233 {}
        impl ClosureGroup___233 {
            fn fn__14060(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update limit".to_string());
            }
        }
        let closure_group = ClosureGroup___233 {};
        let fn__14060 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14060())
        };
        test___201.assert(t___14072, fn__14060.clone());
        test___201.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryEscaping__2583() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___202 = temper_std::testing::Test::new();
        let mut t___14047: SafeIdentifier;
        let mut t___14048: SafeIdentifier;
        let mut t___14049: SqlString;
        let mut t___14050: UpdateQuery;
        let mut t___14051: SqlBuilder;
        let mut t___6839: SqlFragment;
        let q__1847: SqlFragment;
        'ok___17464: {
            'orelse___2781: {
                t___14047 = sid__709("users");
                t___14048 = sid__709("bio");
                t___14049 = SqlString::new("It's a test");
                t___14050 = update(t___14047.clone()).set(t___14048.clone(), SqlPart::new(t___14049.clone()));
                t___14051 = SqlBuilder::new();
                t___14051.append_safe("id = ");
                t___14051.append_int32(1);
                t___6839 = match t___14050.r#where(t___14051.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2781
                };
                q__1847 = t___6839.clone();
                break 'ok___17464;
            }
            q__1847 = panic!();
        }
        let mut t___14058: bool = Some(q__1847.to_string().as_str()) == Some("UPDATE users SET bio = 'It''s a test' WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___234 {}
        impl ClosureGroup___234 {
            fn fn__14046(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update escaping".to_string());
            }
        }
        let closure_group = ClosureGroup___234 {};
        let fn__14046 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14046())
        };
        test___202.assert(t___14058, fn__14046.clone());
        test___202.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryBasic__2585() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___203 = temper_std::testing::Test::new();
        let mut t___14036: SafeIdentifier;
        let mut t___14037: SqlBuilder;
        let mut t___14040: SqlFragment;
        let mut t___6824: SqlFragment;
        let q__1849: SqlFragment;
        'ok___17465: {
            'orelse___2782: {
                t___14036 = sid__709("users");
                t___14037 = SqlBuilder::new();
                t___14037.append_safe("id = ");
                t___14037.append_int32(1);
                t___14040 = t___14037.accumulated();
                t___6824 = match delete_from(t___14036.clone()).r#where(t___14040.clone()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2782
                };
                q__1849 = t___6824.clone();
                break 'ok___17465;
            }
            q__1849 = panic!();
        }
        let mut t___14044: bool = Some(q__1849.to_string().as_str()) == Some("DELETE FROM users WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___235 {}
        impl ClosureGroup___235 {
            fn fn__14035(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete basic".to_string());
            }
        }
        let closure_group = ClosureGroup___235 {};
        let fn__14035 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14035())
        };
        test___203.assert(t___14044, fn__14035.clone());
        test___203.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryMultipleWhere__2587() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___204 = temper_std::testing::Test::new();
        let mut t___14020: SafeIdentifier;
        let mut t___14021: SqlBuilder;
        let mut t___14024: SqlFragment;
        let mut t___14025: DeleteQuery;
        let mut t___14026: SqlBuilder;
        let mut t___6812: SqlFragment;
        let q__1851: SqlFragment;
        'ok___17466: {
            'orelse___2783: {
                t___14020 = sid__709("logs");
                t___14021 = SqlBuilder::new();
                t___14021.append_safe("created_at < ");
                t___14021.append_string("2024-01-01");
                t___14024 = t___14021.accumulated();
                t___14025 = delete_from(t___14020.clone()).r#where(t___14024.clone());
                t___14026 = SqlBuilder::new();
                t___14026.append_safe("level = ");
                t___14026.append_string("debug");
                t___6812 = match t___14025.r#where(t___14026.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2783
                };
                q__1851 = t___6812.clone();
                break 'ok___17466;
            }
            q__1851 = panic!();
        }
        let mut t___14033: bool = Some(q__1851.to_string().as_str()) == Some("DELETE FROM logs WHERE created_at < '2024-01-01' AND level = 'debug'");
        #[derive(Clone)]
        struct ClosureGroup___236 {}
        impl ClosureGroup___236 {
            fn fn__14019(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete multi where".to_string());
            }
        }
        let closure_group = ClosureGroup___236 {};
        let fn__14019 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14019())
        };
        test___204.assert(t___14033, fn__14019.clone());
        test___204.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryBubblesWithoutWhere__2590() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___205 = temper_std::testing::Test::new();
        let didBubble__1853: bool;
        'ok___17467: {
            'orelse___2784: {
                match delete_from(sid__709("users")).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2784
                };
                didBubble__1853 = false;
                break 'ok___17467;
            }
            didBubble__1853 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___237 {}
        impl ClosureGroup___237 {
            fn fn__14015(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete without WHERE should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___237 {};
        let fn__14015 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14015())
        };
        test___205.assert(didBubble__1853, fn__14015.clone());
        test___205.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryOrWhere__2591() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___206 = temper_std::testing::Test::new();
        let mut t___14000: SafeIdentifier;
        let mut t___14001: SqlBuilder;
        let mut t___14004: SqlFragment;
        let mut t___14005: DeleteQuery;
        let mut t___14006: SqlBuilder;
        let mut t___6791: SqlFragment;
        let q__1855: SqlFragment;
        'ok___17468: {
            'orelse___2785: {
                t___14000 = sid__709("sessions");
                t___14001 = SqlBuilder::new();
                t___14001.append_safe("expired = ");
                t___14001.append_boolean(true);
                t___14004 = t___14001.accumulated();
                t___14005 = delete_from(t___14000.clone()).r#where(t___14004.clone());
                t___14006 = SqlBuilder::new();
                t___14006.append_safe("created_at < ");
                t___14006.append_string("2023-01-01");
                t___6791 = match t___14005.or_where(t___14006.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2785
                };
                q__1855 = t___6791.clone();
                break 'ok___17468;
            }
            q__1855 = panic!();
        }
        let mut t___14013: bool = Some(q__1855.to_string().as_str()) == Some("DELETE FROM sessions WHERE expired = TRUE OR created_at < '2023-01-01'");
        #[derive(Clone)]
        struct ClosureGroup___238 {}
        impl ClosureGroup___238 {
            fn fn__13999(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___238 {};
        let fn__13999 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13999())
        };
        test___206.assert(t___14013, fn__13999.clone());
        test___206.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryWithLimit__2594() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___207 = temper_std::testing::Test::new();
        let mut t___13989: SafeIdentifier;
        let mut t___13990: SqlBuilder;
        let mut t___13993: SqlFragment;
        let mut t___6772: DeleteQuery;
        let mut t___6773: SqlFragment;
        let q__1857: SqlFragment;
        'ok___17469: {
            'orelse___2786: {
                t___13989 = sid__709("logs");
                t___13990 = SqlBuilder::new();
                t___13990.append_safe("level = ");
                t___13990.append_string("debug");
                t___13993 = t___13990.accumulated();
                t___6772 = match delete_from(t___13989.clone()).r#where(t___13993.clone()).limit(1000) {
                    Ok(x) => x,
                    _ => break 'orelse___2786
                };
                t___6773 = match t___6772.to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2786
                };
                q__1857 = t___6773.clone();
                break 'ok___17469;
            }
            q__1857 = panic!();
        }
        let mut t___13997: bool = Some(q__1857.to_string().as_str()) == Some("DELETE FROM logs WHERE level = 'debug' LIMIT 1000");
        #[derive(Clone)]
        struct ClosureGroup___239 {}
        impl ClosureGroup___239 {
            fn fn__13988(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete limit".to_string());
            }
        }
        let closure_group = ClosureGroup___239 {};
        let fn__13988 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13988())
        };
        test___207.assert(t___13997, fn__13988.clone());
        test___207.soft_fail_to_hard()
    }
    #[test]
    fn orderByNullsNullsFirst__2596() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___208 = temper_std::testing::Test::new();
        let mut t___13979: SafeIdentifier = sid__709("users");
        let mut t___13980: SafeIdentifier = sid__709("email");
        let mut t___13981: NullsFirst = NullsFirst::new();
        let q__1859: Query = from(t___13979.clone()).order_by_nulls(t___13980.clone(), true, NullsPosition::new(t___13981.clone()));
        let mut t___13986: bool = Some(q__1859.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY email ASC NULLS FIRST");
        #[derive(Clone)]
        struct ClosureGroup___240 {}
        impl ClosureGroup___240 {
            fn fn__13978(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("nulls first".to_string());
            }
        }
        let closure_group = ClosureGroup___240 {};
        let fn__13978 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13978())
        };
        test___208.assert(t___13986, fn__13978.clone());
        test___208.soft_fail_to_hard()
    }
    #[test]
    fn orderByNullsNullsLast__2597() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___209 = temper_std::testing::Test::new();
        let mut t___13969: SafeIdentifier = sid__709("users");
        let mut t___13970: SafeIdentifier = sid__709("score");
        let mut t___13971: NullsLast = NullsLast::new();
        let q__1861: Query = from(t___13969.clone()).order_by_nulls(t___13970.clone(), false, NullsPosition::new(t___13971.clone()));
        let mut t___13976: bool = Some(q__1861.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY score DESC NULLS LAST");
        #[derive(Clone)]
        struct ClosureGroup___241 {}
        impl ClosureGroup___241 {
            fn fn__13968(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("nulls last".to_string());
            }
        }
        let closure_group = ClosureGroup___241 {};
        let fn__13968 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13968())
        };
        test___209.assert(t___13976, fn__13968.clone());
        test___209.soft_fail_to_hard()
    }
    #[test]
    fn mixedOrderByAndOrderByNulls__2598() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___210 = temper_std::testing::Test::new();
        let mut t___13957: SafeIdentifier = sid__709("users");
        let mut t___13958: SafeIdentifier = sid__709("name");
        let q__1863: Query = from(t___13957.clone()).order_by(t___13958.clone(), true).order_by_nulls(sid__709("email"), true, NullsPosition::new(NullsFirst::new()));
        let mut t___13966: bool = Some(q__1863.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC, email ASC NULLS FIRST");
        #[derive(Clone)]
        struct ClosureGroup___242 {}
        impl ClosureGroup___242 {
            fn fn__13956(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mixed order".to_string());
            }
        }
        let closure_group = ClosureGroup___242 {};
        let fn__13956 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13956())
        };
        test___210.assert(t___13966, fn__13956.clone());
        test___210.soft_fail_to_hard()
    }
    #[test]
    fn crossJoin__2599() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___211 = temper_std::testing::Test::new();
        let mut t___13948: SafeIdentifier = sid__709("users");
        let mut t___13949: SafeIdentifier = sid__709("colors");
        let q__1865: Query = from(t___13948.clone()).cross_join(t___13949.clone());
        let mut t___13954: bool = Some(q__1865.to_sql().to_string().as_str()) == Some("SELECT * FROM users CROSS JOIN colors");
        #[derive(Clone)]
        struct ClosureGroup___243 {}
        impl ClosureGroup___243 {
            fn fn__13947(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("cross join".to_string());
            }
        }
        let closure_group = ClosureGroup___243 {};
        let fn__13947 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13947())
        };
        test___211.assert(t___13954, fn__13947.clone());
        test___211.soft_fail_to_hard()
    }
    #[test]
    fn crossJoinCombinedWithOtherJoins__2600() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___212 = temper_std::testing::Test::new();
        let mut t___13934: SafeIdentifier = sid__709("users");
        let mut t___13935: SafeIdentifier = sid__709("orders");
        let mut t___13936: SqlBuilder = SqlBuilder::new();
        t___13936.append_safe("users.id = orders.user_id");
        let mut t___13938: SqlFragment = t___13936.accumulated();
        let q__1867: Query = from(t___13934.clone()).inner_join(t___13935.clone(), t___13938.clone()).cross_join(sid__709("colors"));
        let mut t___13945: bool = Some(q__1867.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id CROSS JOIN colors");
        #[derive(Clone)]
        struct ClosureGroup___244 {}
        impl ClosureGroup___244 {
            fn fn__13933(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("cross + inner join".to_string());
            }
        }
        let closure_group = ClosureGroup___244 {};
        let fn__13933 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13933())
        };
        test___212.assert(t___13945, fn__13933.clone());
        test___212.soft_fail_to_hard()
    }
    #[test]
    fn lockForUpdate__2602() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___213 = temper_std::testing::Test::new();
        let mut t___13920: SafeIdentifier = sid__709("users");
        let mut t___13921: SqlBuilder = SqlBuilder::new();
        t___13921.append_safe("id = ");
        t___13921.append_int32(1);
        let mut t___13924: SqlFragment = t___13921.accumulated();
        let q__1869: Query = from(t___13920.clone()).r#where(t___13924.clone()).lock(LockMode::new(ForUpdate::new()));
        let mut t___13931: bool = Some(q__1869.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id = 1 FOR UPDATE");
        #[derive(Clone)]
        struct ClosureGroup___245 {}
        impl ClosureGroup___245 {
            fn fn__13919(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("for update".to_string());
            }
        }
        let closure_group = ClosureGroup___245 {};
        let fn__13919 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13919())
        };
        test___213.assert(t___13931, fn__13919.clone());
        test___213.soft_fail_to_hard()
    }
    #[test]
    fn lockForShare__2604() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___214 = temper_std::testing::Test::new();
        let mut t___13909: SafeIdentifier = sid__709("users");
        let mut t___13910: SafeIdentifier = sid__709("name");
        let q__1871: Query = from(t___13909.clone()).select([t___13910.clone()]).lock(LockMode::new(ForShare::new()));
        let mut t___13917: bool = Some(q__1871.to_sql().to_string().as_str()) == Some("SELECT name FROM users FOR SHARE");
        #[derive(Clone)]
        struct ClosureGroup___246 {}
        impl ClosureGroup___246 {
            fn fn__13908(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("for share".to_string());
            }
        }
        let closure_group = ClosureGroup___246 {};
        let fn__13908 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13908())
        };
        test___214.assert(t___13917, fn__13908.clone());
        test___214.soft_fail_to_hard()
    }
    #[test]
    fn lockWithFullQuery__2605() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___215 = temper_std::testing::Test::new();
        let mut t___13895: SafeIdentifier;
        let mut t___13896: SqlBuilder;
        let mut t___13899: SqlFragment;
        let mut t___13902: Query;
        let mut t___6696: Query;
        let q__1873: Query;
        'ok___17470: {
            'orelse___2787: {
                t___13895 = sid__709("accounts");
                t___13896 = SqlBuilder::new();
                t___13896.append_safe("id = ");
                t___13896.append_int32(42);
                t___13899 = t___13896.accumulated();
                t___6696 = match from(t___13895.clone()).r#where(t___13899.clone()).limit(1) {
                    Ok(x) => x,
                    _ => break 'orelse___2787
                };
                t___13902 = t___6696.lock(LockMode::new(ForUpdate::new()));
                q__1873 = t___13902.clone();
                break 'ok___17470;
            }
            q__1873 = panic!();
        }
        let mut t___13906: bool = Some(q__1873.to_sql().to_string().as_str()) == Some("SELECT * FROM accounts WHERE id = 42 LIMIT 1 FOR UPDATE");
        #[derive(Clone)]
        struct ClosureGroup___247 {}
        impl ClosureGroup___247 {
            fn fn__13894(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("lock full query".to_string());
            }
        }
        let closure_group = ClosureGroup___247 {};
        let fn__13894 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13894())
        };
        test___215.assert(t___13906, fn__13894.clone());
        test___215.soft_fail_to_hard()
    }
    #[test]
    fn queryBuilderImmutabilityTwoQueriesFromSameBase__2607() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___216 = temper_std::testing::Test::new();
        let mut t___6677: Query;
        let mut t___6680: Query;
        let mut t___13878: SafeIdentifier = sid__709("users");
        let mut t___13879: SqlBuilder = SqlBuilder::new();
        t___13879.append_safe("active = ");
        t___13879.append_boolean(true);
        let mut t___13882: SqlFragment = t___13879.accumulated();
        let base__1875: Query = from(t___13878.clone()).r#where(t___13882.clone());
        let q1__1876: Query;
        'ok___17471: {
            'orelse___2788: {
                t___6677 = match base__1875.limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___2788
                };
                q1__1876 = t___6677.clone();
                break 'ok___17471;
            }
            q1__1876 = panic!();
        }
        let q2__1877: Query;
        'ok___17472: {
            'orelse___2789: {
                t___6680 = match base__1875.limit(20) {
                    Ok(x) => x,
                    _ => break 'orelse___2789
                };
                q2__1877 = t___6680.clone();
                break 'ok___17472;
            }
            q2__1877 = panic!();
        }
        let mut t___13887: bool = Some(q1__1876.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE LIMIT 10");
        #[derive(Clone)]
        struct ClosureGroup___248 {}
        impl ClosureGroup___248 {
            fn fn__13877(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("q1".to_string());
            }
        }
        let closure_group = ClosureGroup___248 {};
        let fn__13877 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13877())
        };
        test___216.assert(t___13887, fn__13877.clone());
        let mut t___13892: bool = Some(q2__1877.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE LIMIT 20");
        #[derive(Clone)]
        struct ClosureGroup___249 {}
        impl ClosureGroup___249 {
            fn fn__13876(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("q2".to_string());
            }
        }
        let closure_group = ClosureGroup___249 {};
        let fn__13876 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13876())
        };
        test___216.assert(t___13892, fn__13876.clone());
        test___216.soft_fail_to_hard()
    }
    #[test]
    fn limitZeroProducesLimit0__2609() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___217 = temper_std::testing::Test::new();
        let mut t___6664: Query;
        let q__1879: Query;
        'ok___17473: {
            'orelse___2790: {
                t___6664 = match from(sid__709("users")).limit(0) {
                    Ok(x) => x,
                    _ => break 'orelse___2790
                };
                q__1879 = t___6664.clone();
                break 'ok___17473;
            }
            q__1879 = panic!();
        }
        let mut t___13874: bool = Some(q__1879.to_sql().to_string().as_str()) == Some("SELECT * FROM users LIMIT 0");
        #[derive(Clone)]
        struct ClosureGroup___250 {}
        impl ClosureGroup___250 {
            fn fn__13869(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("limit 0".to_string());
            }
        }
        let closure_group = ClosureGroup___250 {};
        let fn__13869 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13869())
        };
        test___217.assert(t___13874, fn__13869.clone());
        test___217.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlWithZeroDefaultLimit__2610() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___218 = temper_std::testing::Test::new();
        let mut t___6658: SqlFragment;
        let q__1881: Query = from(sid__709("users"));
        let s__1882: SqlFragment;
        'ok___17474: {
            'orelse___2791: {
                t___6658 = match q__1881.safe_to_sql(0) {
                    Ok(x) => x,
                    _ => break 'orelse___2791
                };
                s__1882 = t___6658.clone();
                break 'ok___17474;
            }
            s__1882 = panic!();
        }
        let mut t___13867: bool = Some(s__1882.to_string().as_str()) == Some("SELECT * FROM users LIMIT 0");
        #[derive(Clone)]
        struct ClosureGroup___251 {}
        impl ClosureGroup___251 {
            fn fn__13863(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("safeToSql 0".to_string());
            }
        }
        let closure_group = ClosureGroup___251 {};
        let fn__13863 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13863())
        };
        test___218.assert(t___13867, fn__13863.clone());
        test___218.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryLimitBubblesOnNegative__2611() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___219 = temper_std::testing::Test::new();
        let mut t___13852: SafeIdentifier;
        let mut t___13853: SafeIdentifier;
        let mut t___13854: SqlString;
        let mut t___13855: UpdateQuery;
        let mut t___13856: SqlBuilder;
        let didBubble__1884: bool;
        'ok___17475: {
            'orelse___2792: {
                t___13852 = sid__709("users");
                t___13853 = sid__709("name");
                t___13854 = SqlString::new("x");
                t___13855 = update(t___13852.clone()).set(t___13853.clone(), SqlPart::new(t___13854.clone()));
                t___13856 = SqlBuilder::new();
                t___13856.append_safe("id = ");
                t___13856.append_int32(1);
                match t___13855.r#where(t___13856.accumulated()).limit(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___2792
                };
                didBubble__1884 = false;
                break 'ok___17475;
            }
            didBubble__1884 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___252 {}
        impl ClosureGroup___252 {
            fn fn__13851(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("UpdateQuery negative limit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___252 {};
        let fn__13851 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13851())
        };
        test___219.assert(didBubble__1884, fn__13851.clone());
        test___219.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryLimitBubblesOnNegative__2613() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___220 = temper_std::testing::Test::new();
        let mut t___13843: SafeIdentifier;
        let mut t___13844: SqlBuilder;
        let mut t___13847: SqlFragment;
        let didBubble__1886: bool;
        'ok___17476: {
            'orelse___2793: {
                t___13843 = sid__709("users");
                t___13844 = SqlBuilder::new();
                t___13844.append_safe("id = ");
                t___13844.append_int32(1);
                t___13847 = t___13844.accumulated();
                match delete_from(t___13843.clone()).r#where(t___13847.clone()).limit(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___2793
                };
                didBubble__1886 = false;
                break 'ok___17476;
            }
            didBubble__1886 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___253 {}
        impl ClosureGroup___253 {
            fn fn__13842(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("DeleteQuery negative limit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___253 {};
        let fn__13842 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13842())
        };
        test___220.assert(didBubble__1886, fn__13842.clone());
        test___220.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryImmutabilityTwoFromSameBase__2615() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___221 = temper_std::testing::Test::new();
        let mut t___6618: SqlFragment;
        let mut t___6619: SqlFragment;
        let mut t___6621: SqlFragment;
        let mut t___6622: SqlFragment;
        let mut t___13812: SafeIdentifier = sid__709("users");
        let mut t___13813: SafeIdentifier = sid__709("name");
        let mut t___13814: SqlString = SqlString::new("Alice");
        let mut t___13815: UpdateQuery = update(t___13812.clone()).set(t___13813.clone(), SqlPart::new(t___13814.clone()));
        let mut t___13816: SqlBuilder = SqlBuilder::new();
        t___13816.append_safe("id = ");
        t___13816.append_int32(1);
        let base__1888: UpdateQuery = t___13815.r#where(t___13816.accumulated());
        let q1__1889: UpdateQuery = base__1888.set(sid__709("age"), SqlPart::new(SqlInt32::new(25)));
        let q2__1890: UpdateQuery = base__1888.set(sid__709("age"), SqlPart::new(SqlInt32::new(30)));
        'ok___17477: {
            'orelse___2794: {
                t___6618 = match q1__1889.to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2794
                };
                t___6619 = t___6618.clone();
                break 'ok___17477;
            }
            t___6619 = panic!();
        }
        let s1__1891: std::sync::Arc<String> = t___6619.to_string();
        'ok___17478: {
            'orelse___2795: {
                t___6621 = match q2__1890.to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2795
                };
                t___6622 = t___6621.clone();
                break 'ok___17478;
            }
            t___6622 = panic!();
        }
        let s2__1892: std::sync::Arc<String> = t___6622.to_string();
        let mut t___13830: bool = temper_core::string::index_of( & s1__1891, "25", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___254 {
            s1__1891: std::sync::Arc<String>
        }
        impl ClosureGroup___254 {
            fn fn__13811(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("q1 should have 25: {}", self.s1__1891.clone()));
            }
        }
        let closure_group = ClosureGroup___254 {
            s1__1891: s1__1891.clone()
        };
        let fn__13811 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13811())
        };
        test___221.assert(t___13830, fn__13811.clone());
        let mut t___13834: bool = temper_core::string::index_of( & s2__1892, "30", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___255 {
            s2__1892: std::sync::Arc<String>
        }
        impl ClosureGroup___255 {
            fn fn__13810(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("q2 should have 30: {}", self.s2__1892.clone()));
            }
        }
        let closure_group = ClosureGroup___255 {
            s2__1892: s2__1892.clone()
        };
        let fn__13810 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13810())
        };
        test___221.assert(t___13834, fn__13810.clone());
        let mut t___13840: bool = ! temper_core::string::index_of( & s1__1891, "30", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___256 {
            s1__1891: std::sync::Arc<String>
        }
        impl ClosureGroup___256 {
            fn fn__13809(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("q1 should NOT have 30: {}", self.s1__1891.clone()));
            }
        }
        let closure_group = ClosureGroup___256 {
            s1__1891: s1__1891.clone()
        };
        let fn__13809 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13809())
        };
        test___221.assert(t___13840, fn__13809.clone());
        test___221.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryImmutability__2617() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___222 = temper_std::testing::Test::new();
        let mut t___6584: SqlFragment;
        let mut t___6585: SqlFragment;
        let mut t___6587: SqlFragment;
        let mut t___6588: SqlFragment;
        let mut t___13778: SafeIdentifier = sid__709("users");
        let mut t___13779: SqlBuilder = SqlBuilder::new();
        t___13779.append_safe("active = ");
        t___13779.append_boolean(false);
        let mut t___13782: SqlFragment = t___13779.accumulated();
        let base__1894: DeleteQuery = delete_from(t___13778.clone()).r#where(t___13782.clone());
        let mut t___13784: SqlBuilder = SqlBuilder::new();
        t___13784.append_safe("age < ");
        t___13784.append_int32(18);
        let q1__1895: DeleteQuery = base__1894.r#where(t___13784.accumulated());
        let mut t___13789: SqlBuilder = SqlBuilder::new();
        t___13789.append_safe("age > ");
        t___13789.append_int32(65);
        let q2__1896: DeleteQuery = base__1894.r#where(t___13789.accumulated());
        'ok___17479: {
            'orelse___2796: {
                t___6584 = match q1__1895.to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2796
                };
                t___6585 = t___6584.clone();
                break 'ok___17479;
            }
            t___6585 = panic!();
        }
        let s1__1897: std::sync::Arc<String> = t___6585.to_string();
        'ok___17480: {
            'orelse___2797: {
                t___6587 = match q2__1896.to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2797
                };
                t___6588 = t___6587.clone();
                break 'ok___17480;
            }
            t___6588 = panic!();
        }
        let s2__1898: std::sync::Arc<String> = t___6588.to_string();
        let mut t___13797: bool = temper_core::string::index_of( & s1__1897, "age < 18", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___257 {
            s1__1897: std::sync::Arc<String>
        }
        impl ClosureGroup___257 {
            fn fn__13777(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("q1: {}", self.s1__1897.clone()));
            }
        }
        let closure_group = ClosureGroup___257 {
            s1__1897: s1__1897.clone()
        };
        let fn__13777 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13777())
        };
        test___222.assert(t___13797, fn__13777.clone());
        let mut t___13801: bool = temper_core::string::index_of( & s2__1898, "age > 65", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___258 {
            s2__1898: std::sync::Arc<String>
        }
        impl ClosureGroup___258 {
            fn fn__13776(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("q2: {}", self.s2__1898.clone()));
            }
        }
        let closure_group = ClosureGroup___258 {
            s2__1898: s2__1898.clone()
        };
        let fn__13776 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13776())
        };
        test___222.assert(t___13801, fn__13776.clone());
        let mut t___13807: bool = ! temper_core::string::index_of( & s1__1897, "age > 65", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___259 {
            s1__1897: std::sync::Arc<String>
        }
        impl ClosureGroup___259 {
            fn fn__13775(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("q1 should not have q2 condition: {}", self.s1__1897.clone()));
            }
        }
        let closure_group = ClosureGroup___259 {
            s1__1897: s1__1897.clone()
        };
        let fn__13775 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13775())
        };
        test___222.assert(t___13807, fn__13775.clone());
        test___222.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierAcceptsValidNames__2621() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___229 = temper_std::testing::Test::new();
        let mut t___6561: SafeIdentifier;
        let id__1946: SafeIdentifier;
        'ok___17481: {
            'orelse___2798: {
                t___6561 = match safe_identifier("user_name") {
                    Ok(x) => x,
                    _ => break 'orelse___2798
                };
                id__1946 = t___6561.clone();
                break 'ok___17481;
            }
            id__1946 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13773: bool = Some(id__1946.sql_value().as_str()) == Some("user_name");
        #[derive(Clone)]
        struct ClosureGroup___260 {}
        impl ClosureGroup___260 {
            fn fn__13770(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("value should round-trip".to_string());
            }
        }
        let closure_group = ClosureGroup___260 {};
        let fn__13770 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13770())
        };
        test___229.assert(t___13773, fn__13770.clone());
        test___229.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsEmptyString__2622() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___230 = temper_std::testing::Test::new();
        let didBubble__1948: bool;
        'ok___17482: {
            'orelse___2799: {
                match safe_identifier("") {
                    Ok(x) => x,
                    _ => break 'orelse___2799
                };
                didBubble__1948 = false;
                break 'ok___17482;
            }
            didBubble__1948 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___261 {}
        impl ClosureGroup___261 {
            fn fn__13767(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty string should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___261 {};
        let fn__13767 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13767())
        };
        test___230.assert(didBubble__1948, fn__13767.clone());
        test___230.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsLeadingDigit__2623() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___231 = temper_std::testing::Test::new();
        let didBubble__1950: bool;
        'ok___17483: {
            'orelse___2800: {
                match safe_identifier("1col") {
                    Ok(x) => x,
                    _ => break 'orelse___2800
                };
                didBubble__1950 = false;
                break 'ok___17483;
            }
            didBubble__1950 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___262 {}
        impl ClosureGroup___262 {
            fn fn__13764(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("leading digit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___262 {};
        let fn__13764 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13764())
        };
        test___231.assert(didBubble__1950, fn__13764.clone());
        test___231.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsSqlMetacharacters__2624() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___232 = temper_std::testing::Test::new();
        let cases__1952: temper_core::List<std::sync::Arc<String>> = std::sync::Arc::new(vec![std::sync::Arc::new("name); DROP TABLE".to_string()), std::sync::Arc::new("col'".to_string()), std::sync::Arc::new("a b".to_string()), std::sync::Arc::new("a-b".to_string()), std::sync::Arc::new("a.b".to_string()), std::sync::Arc::new("a;b".to_string())]);
        #[derive(Clone)]
        struct ClosureGroup___263 {
            test___232: temper_std::testing::Test
        }
        impl ClosureGroup___263 {
            fn fn__13761(& self, c__1953: impl temper_core::ToArcString) {
                let c__1953 = c__1953.to_arc_string();
                let didBubble__1954: bool;
                'ok___17484: {
                    'orelse___2801: {
                        match safe_identifier(c__1953.clone()) {
                            Ok(x) => x,
                            _ => break 'orelse___2801
                        };
                        didBubble__1954 = false;
                        break 'ok___17484;
                    }
                    didBubble__1954 = true;
                }
                #[derive(Clone)]
                struct ClosureGroup___264 {
                    c__1953: std::sync::Arc<String>
                }
                impl ClosureGroup___264 {
                    fn fn__13758(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject: {}", self.c__1953.clone()));
                    }
                }
                let closure_group = ClosureGroup___264 {
                    c__1953: c__1953.clone()
                };
                let fn__13758 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__13758())
                };
                self.test___232.assert(didBubble__1954, fn__13758.clone());
            }
        }
        let closure_group = ClosureGroup___263 {
            test___232: test___232.clone()
        };
        let fn__13761 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1953: std::sync::Arc<String> | closure_group.fn__13761(c__1953))
        };
        temper_core::listed::list_for_each( & cases__1952, & ( * fn__13761.clone()));
        test___232.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupFound__2625() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___233 = temper_std::testing::Test::new();
        let mut t___6538: SafeIdentifier;
        let mut t___6539: SafeIdentifier;
        let mut t___6540: SafeIdentifier;
        let mut t___6541: SafeIdentifier;
        let mut t___6544: SafeIdentifier;
        let mut t___6545: SafeIdentifier;
        let mut t___6549: FieldDef;
        'ok___17485: {
            'orelse___2802: {
                t___6538 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___2802
                };
                t___6539 = t___6538.clone();
                break 'ok___17485;
            }
            t___6539 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___17486: {
            'orelse___2803: {
                t___6540 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___2803
                };
                t___6541 = t___6540.clone();
                break 'ok___17486;
            }
            t___6541 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13748: StringField = StringField::new();
        let mut t___13749: FieldDef = FieldDef::new(t___6541.clone(), FieldType::new(t___13748.clone()), false, None, false);
        'ok___17487: {
            'orelse___2804: {
                t___6544 = match safe_identifier("age") {
                    Ok(x) => x,
                    _ => break 'orelse___2804
                };
                t___6545 = t___6544.clone();
                break 'ok___17487;
            }
            t___6545 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13750: IntField = IntField::new();
        let mut t___13751: FieldDef = FieldDef::new(t___6545.clone(), FieldType::new(t___13750.clone()), false, None, false);
        let td__1956: TableDef = TableDef::new(t___6539.clone(), [t___13749.clone(), t___13751.clone()], None);
        let f__1957: FieldDef;
        'ok___17488: {
            'orelse___2805: {
                t___6549 = match td__1956.field("age") {
                    Ok(x) => x,
                    _ => break 'orelse___2805
                };
                f__1957 = t___6549.clone();
                break 'ok___17488;
            }
            f__1957 = panic!();
        }
        let mut t___13756: bool = Some(f__1957.name().sql_value().as_str()) == Some("age");
        #[derive(Clone)]
        struct ClosureGroup___265 {}
        impl ClosureGroup___265 {
            fn fn__13747(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should find age field".to_string());
            }
        }
        let closure_group = ClosureGroup___265 {};
        let fn__13747 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13747())
        };
        test___233.assert(t___13756, fn__13747.clone());
        test___233.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupNotFoundBubbles__2626() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___234 = temper_std::testing::Test::new();
        let mut t___6529: SafeIdentifier;
        let mut t___6530: SafeIdentifier;
        let mut t___6531: SafeIdentifier;
        let mut t___6532: SafeIdentifier;
        'ok___17489: {
            'orelse___2806: {
                t___6529 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___2806
                };
                t___6530 = t___6529.clone();
                break 'ok___17489;
            }
            t___6530 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___17490: {
            'orelse___2807: {
                t___6531 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___2807
                };
                t___6532 = t___6531.clone();
                break 'ok___17490;
            }
            t___6532 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13742: StringField = StringField::new();
        let mut t___13743: FieldDef = FieldDef::new(t___6532.clone(), FieldType::new(t___13742.clone()), false, None, false);
        let td__1959: TableDef = TableDef::new(t___6530.clone(), [t___13743.clone()], None);
        let didBubble__1960: bool;
        'ok___17491: {
            'orelse___2808: {
                match td__1959.field("nonexistent") {
                    Ok(x) => x,
                    _ => break 'orelse___2808
                };
                didBubble__1960 = false;
                break 'ok___17491;
            }
            didBubble__1960 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___266 {}
        impl ClosureGroup___266 {
            fn fn__13741(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("unknown field should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___266 {};
        let fn__13741 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13741())
        };
        test___234.assert(didBubble__1960, fn__13741.clone());
        test___234.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefNullableFlag__2627() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___235 = temper_std::testing::Test::new();
        let mut t___6517: SafeIdentifier;
        let mut t___6518: SafeIdentifier;
        let mut t___6521: SafeIdentifier;
        let mut t___6522: SafeIdentifier;
        'ok___17492: {
            'orelse___2809: {
                t___6517 = match safe_identifier("email") {
                    Ok(x) => x,
                    _ => break 'orelse___2809
                };
                t___6518 = t___6517.clone();
                break 'ok___17492;
            }
            t___6518 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13730: StringField = StringField::new();
        let required__1962: FieldDef = FieldDef::new(t___6518.clone(), FieldType::new(t___13730.clone()), false, None, false);
        'ok___17493: {
            'orelse___2810: {
                t___6521 = match safe_identifier("bio") {
                    Ok(x) => x,
                    _ => break 'orelse___2810
                };
                t___6522 = t___6521.clone();
                break 'ok___17493;
            }
            t___6522 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13732: StringField = StringField::new();
        let optional__1963: FieldDef = FieldDef::new(t___6522.clone(), FieldType::new(t___13732.clone()), true, None, false);
        let mut t___13736: bool = ! required__1962.nullable();
        #[derive(Clone)]
        struct ClosureGroup___267 {}
        impl ClosureGroup___267 {
            fn fn__13729(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("required field should not be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___267 {};
        let fn__13729 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13729())
        };
        test___235.assert(t___13736, fn__13729.clone());
        let mut t___13738: bool = optional__1963.nullable();
        #[derive(Clone)]
        struct ClosureGroup___268 {}
        impl ClosureGroup___268 {
            fn fn__13728(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("optional field should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___268 {};
        let fn__13728 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13728())
        };
        test___235.assert(t___13738, fn__13728.clone());
        test___235.soft_fail_to_hard()
    }
    #[test]
    fn pkNameDefaultsToIdWhenPrimaryKeyIsNull__2628() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___236 = temper_std::testing::Test::new();
        let mut t___6508: SafeIdentifier;
        let mut t___6509: SafeIdentifier;
        let mut t___6510: SafeIdentifier;
        let mut t___6511: SafeIdentifier;
        'ok___17494: {
            'orelse___2811: {
                t___6508 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___2811
                };
                t___6509 = t___6508.clone();
                break 'ok___17494;
            }
            t___6509 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___17495: {
            'orelse___2812: {
                t___6510 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___2812
                };
                t___6511 = t___6510.clone();
                break 'ok___17495;
            }
            t___6511 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13721: StringField = StringField::new();
        let mut t___13722: FieldDef = FieldDef::new(t___6511.clone(), FieldType::new(t___13721.clone()), false, None, false);
        let td__1965: TableDef = TableDef::new(t___6509.clone(), [t___13722.clone()], None);
        let mut t___13726: bool = Some(td__1965.pk_name().as_str()) == Some("id");
        #[derive(Clone)]
        struct ClosureGroup___269 {}
        impl ClosureGroup___269 {
            fn fn__13720(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("default pk should be id".to_string());
            }
        }
        let closure_group = ClosureGroup___269 {};
        let fn__13720 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13720())
        };
        test___236.assert(t___13726, fn__13720.clone());
        test___236.soft_fail_to_hard()
    }
    #[test]
    fn pkNameReturnsCustomPrimaryKey__2629() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___237 = temper_std::testing::Test::new();
        let mut t___6496: SafeIdentifier;
        let mut t___6497: SafeIdentifier;
        let mut t___6498: SafeIdentifier;
        let mut t___6499: SafeIdentifier;
        let mut t___6502: SafeIdentifier;
        let mut t___6503: SafeIdentifier;
        'ok___17496: {
            'orelse___2813: {
                t___6496 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___2813
                };
                t___6497 = t___6496.clone();
                break 'ok___17496;
            }
            t___6497 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___17497: {
            'orelse___2814: {
                t___6498 = match safe_identifier("user_id") {
                    Ok(x) => x,
                    _ => break 'orelse___2814
                };
                t___6499 = t___6498.clone();
                break 'ok___17497;
            }
            t___6499 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13713: IntField = IntField::new();
        let mut t___6504: temper_core::List<FieldDef> = std::sync::Arc::new(vec![FieldDef::new(t___6499.clone(), FieldType::new(t___13713.clone()), false, None, false)]);
        'ok___17498: {
            'orelse___2815: {
                t___6502 = match safe_identifier("user_id") {
                    Ok(x) => x,
                    _ => break 'orelse___2815
                };
                t___6503 = t___6502.clone();
                break 'ok___17498;
            }
            t___6503 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let td__1967: TableDef = TableDef::new(t___6497.clone(), t___6504.clone(), Some(t___6503.clone()));
        let mut t___13718: bool = Some(td__1967.pk_name().as_str()) == Some("user_id");
        #[derive(Clone)]
        struct ClosureGroup___270 {}
        impl ClosureGroup___270 {
            fn fn__13712(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("custom pk should be user_id".to_string());
            }
        }
        let closure_group = ClosureGroup___270 {};
        let fn__13712 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13712())
        };
        test___237.assert(t___13718, fn__13712.clone());
        test___237.soft_fail_to_hard()
    }
    #[test]
    fn timestampsReturnsTwoDateFieldDefs__2630() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___238 = temper_std::testing::Test::new();
        let mut t___6472: temper_core::List<FieldDef>;
        let ts__1969: temper_core::List<FieldDef>;
        'ok___17499: {
            'orelse___2816: {
                t___6472 = match timestamps() {
                    Ok(x) => x,
                    _ => break 'orelse___2816
                };
                ts__1969 = t___6472.clone();
                break 'ok___17499;
            }
            ts__1969 = panic!();
        }
        let mut t___13680: bool = Some(temper_core::ListedTrait::len( & ts__1969)) == Some(2);
        #[derive(Clone)]
        struct ClosureGroup___271 {}
        impl ClosureGroup___271 {
            fn fn__13677(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should return 2 fields".to_string());
            }
        }
        let closure_group = ClosureGroup___271 {};
        let fn__13677 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13677())
        };
        test___238.assert(t___13680, fn__13677.clone());
        let mut t___13686: bool = Some(temper_core::ListedTrait::get( & ts__1969, 0).name().sql_value().as_str()) == Some("inserted_at");
        #[derive(Clone)]
        struct ClosureGroup___272 {}
        impl ClosureGroup___272 {
            fn fn__13676(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("first should be inserted_at".to_string());
            }
        }
        let closure_group = ClosureGroup___272 {};
        let fn__13676 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13676())
        };
        test___238.assert(t___13686, fn__13676.clone());
        let mut t___13692: bool = Some(temper_core::ListedTrait::get( & ts__1969, 1).name().sql_value().as_str()) == Some("updated_at");
        #[derive(Clone)]
        struct ClosureGroup___273 {}
        impl ClosureGroup___273 {
            fn fn__13675(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("second should be updated_at".to_string());
            }
        }
        let closure_group = ClosureGroup___273 {};
        let fn__13675 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13675())
        };
        test___238.assert(t___13692, fn__13675.clone());
        let mut t___13695: bool = temper_core::ListedTrait::get( & ts__1969, 0).nullable();
        #[derive(Clone)]
        struct ClosureGroup___274 {}
        impl ClosureGroup___274 {
            fn fn__13674(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("inserted_at should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___274 {};
        let fn__13674 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13674())
        };
        test___238.assert(t___13695, fn__13674.clone());
        let mut t___13699: bool = temper_core::ListedTrait::get( & ts__1969, 1).nullable();
        #[derive(Clone)]
        struct ClosureGroup___275 {}
        impl ClosureGroup___275 {
            fn fn__13673(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("updated_at should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___275 {};
        let fn__13673 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13673())
        };
        test___238.assert(t___13699, fn__13673.clone());
        let mut t___13705: bool = ! temper_core::ListedTrait::get( & ts__1969, 0).default_value().is_none();
        #[derive(Clone)]
        struct ClosureGroup___276 {}
        impl ClosureGroup___276 {
            fn fn__13672(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("inserted_at should have default".to_string());
            }
        }
        let closure_group = ClosureGroup___276 {};
        let fn__13672 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13672())
        };
        test___238.assert(t___13705, fn__13672.clone());
        let mut t___13710: bool = ! temper_core::ListedTrait::get( & ts__1969, 1).default_value().is_none();
        #[derive(Clone)]
        struct ClosureGroup___277 {}
        impl ClosureGroup___277 {
            fn fn__13671(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("updated_at should have default".to_string());
            }
        }
        let closure_group = ClosureGroup___277 {};
        let fn__13671 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13671())
        };
        test___238.assert(t___13710, fn__13671.clone());
        test___238.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefDefaultValueField__2631() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___239 = temper_std::testing::Test::new();
        let mut t___6459: SafeIdentifier;
        let mut t___6460: SafeIdentifier;
        let mut t___6464: SafeIdentifier;
        let mut t___6465: SafeIdentifier;
        'ok___17500: {
            'orelse___2817: {
                t___6459 = match safe_identifier("status") {
                    Ok(x) => x,
                    _ => break 'orelse___2817
                };
                t___6460 = t___6459.clone();
                break 'ok___17500;
            }
            t___6460 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13658: StringField = StringField::new();
        let mut t___13659: SqlDefault = SqlDefault::new();
        let withDefault__1971: FieldDef = FieldDef::new(t___6460.clone(), FieldType::new(t___13658.clone()), false, Some(SqlPart::new(t___13659.clone())), false);
        'ok___17501: {
            'orelse___2818: {
                t___6464 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___2818
                };
                t___6465 = t___6464.clone();
                break 'ok___17501;
            }
            t___6465 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13661: StringField = StringField::new();
        let withoutDefault__1972: FieldDef = FieldDef::new(t___6465.clone(), FieldType::new(t___13661.clone()), false, None, false);
        let mut t___13665: bool = ! withDefault__1971.default_value().is_none();
        #[derive(Clone)]
        struct ClosureGroup___278 {}
        impl ClosureGroup___278 {
            fn fn__13657(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have default".to_string());
            }
        }
        let closure_group = ClosureGroup___278 {};
        let fn__13657 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13657())
        };
        test___239.assert(t___13665, fn__13657.clone());
        let mut t___13669: bool = withoutDefault__1972.default_value().is_none();
        #[derive(Clone)]
        struct ClosureGroup___279 {}
        impl ClosureGroup___279 {
            fn fn__13656(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should not have default".to_string());
            }
        }
        let closure_group = ClosureGroup___279 {};
        let fn__13656 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13656())
        };
        test___239.assert(t___13669, fn__13656.clone());
        test___239.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefVirtualFlag__2632() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___240 = temper_std::testing::Test::new();
        let mut t___6447: SafeIdentifier;
        let mut t___6448: SafeIdentifier;
        let mut t___6451: SafeIdentifier;
        let mut t___6452: SafeIdentifier;
        'ok___17502: {
            'orelse___2819: {
                t___6447 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___2819
                };
                t___6448 = t___6447.clone();
                break 'ok___17502;
            }
            t___6448 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13645: StringField = StringField::new();
        let normal__1974: FieldDef = FieldDef::new(t___6448.clone(), FieldType::new(t___13645.clone()), false, None, false);
        'ok___17503: {
            'orelse___2820: {
                t___6451 = match safe_identifier("full_name") {
                    Ok(x) => x,
                    _ => break 'orelse___2820
                };
                t___6452 = t___6451.clone();
                break 'ok___17503;
            }
            t___6452 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13647: StringField = StringField::new();
        let virt__1975: FieldDef = FieldDef::new(t___6452.clone(), FieldType::new(t___13647.clone()), true, None, true);
        let mut t___13651: bool = ! normal__1974.r#virtual();
        #[derive(Clone)]
        struct ClosureGroup___280 {}
        impl ClosureGroup___280 {
            fn fn__13644(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("normal field should not be virtual".to_string());
            }
        }
        let closure_group = ClosureGroup___280 {};
        let fn__13644 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13644())
        };
        test___240.assert(t___13651, fn__13644.clone());
        let mut t___13653: bool = virt__1975.r#virtual();
        #[derive(Clone)]
        struct ClosureGroup___281 {}
        impl ClosureGroup___281 {
            fn fn__13643(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("virtual field should be virtual".to_string());
            }
        }
        let closure_group = ClosureGroup___281 {};
        let fn__13643 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13643())
        };
        test___240.assert(t___13653, fn__13643.clone());
        test___240.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierAcceptsSingleCharacterNames__2633() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___241 = temper_std::testing::Test::new();
        let mut t___6439: SafeIdentifier;
        let mut t___6443: SafeIdentifier;
        let a__1977: SafeIdentifier;
        'ok___17504: {
            'orelse___2821: {
                t___6439 = match safe_identifier("a") {
                    Ok(x) => x,
                    _ => break 'orelse___2821
                };
                a__1977 = t___6439.clone();
                break 'ok___17504;
            }
            a__1977 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13637: bool = Some(a__1977.sql_value().as_str()) == Some("a");
        #[derive(Clone)]
        struct ClosureGroup___282 {}
        impl ClosureGroup___282 {
            fn fn__13634(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("single letter should work".to_string());
            }
        }
        let closure_group = ClosureGroup___282 {};
        let fn__13634 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13634())
        };
        test___241.assert(t___13637, fn__13634.clone());
        let u__1978: SafeIdentifier;
        'ok___17505: {
            'orelse___2822: {
                t___6443 = match safe_identifier("_") {
                    Ok(x) => x,
                    _ => break 'orelse___2822
                };
                u__1978 = t___6443.clone();
                break 'ok___17505;
            }
            u__1978 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13641: bool = Some(u__1978.sql_value().as_str()) == Some("_");
        #[derive(Clone)]
        struct ClosureGroup___283 {}
        impl ClosureGroup___283 {
            fn fn__13633(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("single underscore should work".to_string());
            }
        }
        let closure_group = ClosureGroup___283 {};
        let fn__13633 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13633())
        };
        test___241.assert(t___13641, fn__13633.clone());
        test___241.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierAcceptsAllUnderscoreNames__2634() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___242 = temper_std::testing::Test::new();
        let mut t___6435: SafeIdentifier;
        let id__1980: SafeIdentifier;
        'ok___17506: {
            'orelse___2823: {
                t___6435 = match safe_identifier("___") {
                    Ok(x) => x,
                    _ => break 'orelse___2823
                };
                id__1980 = t___6435.clone();
                break 'ok___17506;
            }
            id__1980 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___13631: bool = Some(id__1980.sql_value().as_str()) == Some("___");
        #[derive(Clone)]
        struct ClosureGroup___284 {}
        impl ClosureGroup___284 {
            fn fn__13628(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("all underscores should work".to_string());
            }
        }
        let closure_group = ClosureGroup___284 {};
        let fn__13628 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13628())
        };
        test___242.assert(t___13631, fn__13628.clone());
        test___242.soft_fail_to_hard()
    }
    #[test]
    fn tableDefWithEmptyFieldList__2635() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___243 = temper_std::testing::Test::new();
        let mut t___6430: SafeIdentifier;
        let mut t___6431: SafeIdentifier;
        'ok___17507: {
            'orelse___2824: {
                t___6430 = match safe_identifier("empty") {
                    Ok(x) => x,
                    _ => break 'orelse___2824
                };
                t___6431 = t___6430.clone();
                break 'ok___17507;
            }
            t___6431 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let tbl__1982: TableDef = TableDef::new(t___6431.clone(), [], None);
        let didBubble__1983: bool;
        'ok___17508: {
            'orelse___2825: {
                match tbl__1982.field("anything") {
                    Ok(x) => x,
                    _ => break 'orelse___2825
                };
                didBubble__1983 = false;
                break 'ok___17508;
            }
            didBubble__1983 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___285 {}
        impl ClosureGroup___285 {
            fn fn__13624(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("field lookup on empty table should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___285 {};
        let fn__13624 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13624())
        };
        test___243.assert(didBubble__1983, fn__13624.clone());
        test___243.soft_fail_to_hard()
    }
    #[test]
    fn stringEscaping__2636() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___247 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___286 {}
        impl ClosureGroup___286 {
            fn build__2113(& self, name__2115: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__2115 = name__2115.to_arc_string();
                let mut t___13606: SqlBuilder = SqlBuilder::new();
                t___13606.append_safe("select * from hi where name = ");
                t___13606.append_string(name__2115.clone());
                return t___13606.accumulated().to_string();
            }
            fn buildWrong__2114(& self, name__2117: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__2117 = name__2117.to_arc_string();
                return std::sync::Arc::new(format!("select * from hi where name = '{}'", name__2117.clone()));
            }
        }
        let closure_group = ClosureGroup___286 {};
        let build__2113 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__2115: std::sync::Arc<String> | closure_group.build__2113(name__2115))
        };
        let buildWrong__2114 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__2117: std::sync::Arc<String> | closure_group.buildWrong__2114(name__2117))
        };
        let actual___2638: std::sync::Arc<String> = build__2113(std::sync::Arc::new("world".to_string()));
        let mut t___13616: bool = Some(actual___2638.as_str()) == Some("select * from hi where name = 'world'");
        #[derive(Clone)]
        struct ClosureGroup___287 {
            actual___2638: std::sync::Arc<String>
        }
        impl ClosureGroup___287 {
            fn fn__13613(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(\"world\") == (select * from hi where name = 'world') not ({})", self.actual___2638.clone()));
            }
        }
        let closure_group = ClosureGroup___287 {
            actual___2638: actual___2638.clone()
        };
        let fn__13613 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13613())
        };
        test___247.assert(t___13616, fn__13613.clone());
        let bobbyTables__2119: std::sync::Arc<String> = std::sync::Arc::new("Robert'); drop table hi;--".to_string());
        let actual___2640: std::sync::Arc<String> = build__2113(std::sync::Arc::new("Robert'); drop table hi;--".to_string()));
        let mut t___13620: bool = Some(actual___2640.as_str()) == Some("select * from hi where name = 'Robert''); drop table hi;--'");
        #[derive(Clone)]
        struct ClosureGroup___288 {
            actual___2640: std::sync::Arc<String>
        }
        impl ClosureGroup___288 {
            fn fn__13612(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(bobbyTables) == (select * from hi where name = 'Robert''); drop table hi;--') not ({})", self.actual___2640.clone()));
            }
        }
        let closure_group = ClosureGroup___288 {
            actual___2640: actual___2640.clone()
        };
        let fn__13612 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13612())
        };
        test___247.assert(t___13620, fn__13612.clone());
        #[derive(Clone)]
        struct ClosureGroup___289 {}
        impl ClosureGroup___289 {
            fn fn__13611(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')".to_string());
            }
        }
        let closure_group = ClosureGroup___289 {};
        let fn__13611 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13611())
        };
        test___247.assert(true, fn__13611.clone());
        test___247.soft_fail_to_hard()
    }
    #[test]
    fn stringEdgeCases__2644() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___248 = temper_std::testing::Test::new();
        let mut t___13574: SqlBuilder = SqlBuilder::new();
        t___13574.append_safe("v = ");
        t___13574.append_string("");
        let actual___2645: std::sync::Arc<String> = t___13574.accumulated().to_string();
        let mut t___13580: bool = Some(actual___2645.as_str()) == Some("v = ''");
        #[derive(Clone)]
        struct ClosureGroup___290 {
            actual___2645: std::sync::Arc<String>
        }
        impl ClosureGroup___290 {
            fn fn__13573(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"\").toString() == (v = '') not ({})", self.actual___2645.clone()));
            }
        }
        let closure_group = ClosureGroup___290 {
            actual___2645: actual___2645.clone()
        };
        let fn__13573 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13573())
        };
        test___248.assert(t___13580, fn__13573.clone());
        let mut t___13582: SqlBuilder = SqlBuilder::new();
        t___13582.append_safe("v = ");
        t___13582.append_string("a''b");
        let actual___2648: std::sync::Arc<String> = t___13582.accumulated().to_string();
        let mut t___13588: bool = Some(actual___2648.as_str()) == Some("v = 'a''''b'");
        #[derive(Clone)]
        struct ClosureGroup___291 {
            actual___2648: std::sync::Arc<String>
        }
        impl ClosureGroup___291 {
            fn fn__13572(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (v = 'a''''b') not ({})", self.actual___2648.clone()));
            }
        }
        let closure_group = ClosureGroup___291 {
            actual___2648: actual___2648.clone()
        };
        let fn__13572 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13572())
        };
        test___248.assert(t___13588, fn__13572.clone());
        let mut t___13590: SqlBuilder = SqlBuilder::new();
        t___13590.append_safe("v = ");
        t___13590.append_string("Hello 世界");
        let actual___2651: std::sync::Arc<String> = t___13590.accumulated().to_string();
        let mut t___13596: bool = Some(actual___2651.as_str()) == Some("v = 'Hello 世界'");
        #[derive(Clone)]
        struct ClosureGroup___292 {
            actual___2651: std::sync::Arc<String>
        }
        impl ClosureGroup___292 {
            fn fn__13571(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Hello 世界\").toString() == (v = 'Hello 世界') not ({})", self.actual___2651.clone()));
            }
        }
        let closure_group = ClosureGroup___292 {
            actual___2651: actual___2651.clone()
        };
        let fn__13571 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13571())
        };
        test___248.assert(t___13596, fn__13571.clone());
        let mut t___13598: SqlBuilder = SqlBuilder::new();
        t___13598.append_safe("v = ");
        t___13598.append_string("Line1\x0aLine2");
        let actual___2654: std::sync::Arc<String> = t___13598.accumulated().to_string();
        let mut t___13604: bool = Some(actual___2654.as_str()) == Some("v = 'Line1\x0aLine2'");
        #[derive(Clone)]
        struct ClosureGroup___293 {
            actual___2654: std::sync::Arc<String>
        }
        impl ClosureGroup___293 {
            fn fn__13570(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Line1\\nLine2\").toString() == (v = 'Line1\x0aLine2') not ({})", self.actual___2654.clone()));
            }
        }
        let closure_group = ClosureGroup___293 {
            actual___2654: actual___2654.clone()
        };
        let fn__13570 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13570())
        };
        test___248.assert(t___13604, fn__13570.clone());
        test___248.soft_fail_to_hard()
    }
    #[test]
    fn numbersAndBooleans__2657() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___249 = temper_std::testing::Test::new();
        let mut t___6375: temper_std::temporal::Date;
        let mut t___13545: SqlBuilder = SqlBuilder::new();
        t___13545.append_safe("select ");
        t___13545.append_int32(42);
        t___13545.append_safe(", ");
        t___13545.append_int64(43);
        t___13545.append_safe(", ");
        t___13545.append_float64(19.99f64);
        t___13545.append_safe(", ");
        t___13545.append_boolean(true);
        t___13545.append_safe(", ");
        t___13545.append_boolean(false);
        let actual___2658: std::sync::Arc<String> = t___13545.accumulated().to_string();
        let mut t___13559: bool = Some(actual___2658.as_str()) == Some("select 42, 43, 19.99, TRUE, FALSE");
        #[derive(Clone)]
        struct ClosureGroup___294 {
            actual___2658: std::sync::Arc<String>
        }
        impl ClosureGroup___294 {
            fn fn__13544(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, 42, \", \", \\interpolate, 43, \", \", \\interpolate, 19.99, \", \", \\interpolate, true, \", \", \\interpolate, false).toString() == (select 42, 43, 19.99, TRUE, FALSE) not ({})", self.actual___2658.clone()));
            }
        }
        let closure_group = ClosureGroup___294 {
            actual___2658: actual___2658.clone()
        };
        let fn__13544 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13544())
        };
        test___249.assert(t___13559, fn__13544.clone());
        let date__2122: temper_std::temporal::Date;
        'ok___17509: {
            'orelse___2826: {
                t___6375 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___2826
                };
                date__2122 = t___6375.clone();
                break 'ok___17509;
            }
            date__2122 = panic!();
        }
        let mut t___13561: SqlBuilder = SqlBuilder::new();
        t___13561.append_safe("insert into t values (");
        t___13561.append_date(date__2122.clone());
        t___13561.append_safe(")");
        let actual___2661: std::sync::Arc<String> = t___13561.accumulated().to_string();
        let mut t___13568: bool = Some(actual___2661.as_str()) == Some("insert into t values ('2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___295 {
            actual___2661: std::sync::Arc<String>
        }
        impl ClosureGroup___295 {
            fn fn__13543(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"insert into t values (\", \\interpolate, date, \")\").toString() == (insert into t values ('2024-12-25')) not ({})", self.actual___2661.clone()));
            }
        }
        let closure_group = ClosureGroup___295 {
            actual___2661: actual___2661.clone()
        };
        let fn__13543 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13543())
        };
        test___249.assert(t___13568, fn__13543.clone());
        test___249.soft_fail_to_hard()
    }
    #[test]
    fn lists__2664() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___250 = temper_std::testing::Test::new();
        let mut t___6347: temper_std::temporal::Date;
        let mut t___6348: temper_std::temporal::Date;
        let mut t___6349: temper_std::temporal::Date;
        let mut t___6350: temper_std::temporal::Date;
        let mut t___13489: SqlBuilder = SqlBuilder::new();
        t___13489.append_safe("v IN (");
        t___13489.append_string_list(temper_core::ToListed::to_listed([std::sync::Arc::new("a".to_string()), std::sync::Arc::new("b".to_string()), std::sync::Arc::new("c'd".to_string())]));
        t___13489.append_safe(")");
        let actual___2665: std::sync::Arc<String> = t___13489.accumulated().to_string();
        let mut t___13496: bool = Some(actual___2665.as_str()) == Some("v IN ('a', 'b', 'c''d')");
        #[derive(Clone)]
        struct ClosureGroup___296 {
            actual___2665: std::sync::Arc<String>
        }
        impl ClosureGroup___296 {
            fn fn__13488(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (v IN ('a', 'b', 'c''d')) not ({})", self.actual___2665.clone()));
            }
        }
        let closure_group = ClosureGroup___296 {
            actual___2665: actual___2665.clone()
        };
        let fn__13488 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13488())
        };
        test___250.assert(t___13496, fn__13488.clone());
        let mut t___13498: SqlBuilder = SqlBuilder::new();
        t___13498.append_safe("v IN (");
        t___13498.append_int32_list(temper_core::ToListed::to_listed([1, 2, 3]));
        t___13498.append_safe(")");
        let actual___2668: std::sync::Arc<String> = t___13498.accumulated().to_string();
        let mut t___13505: bool = Some(actual___2668.as_str()) == Some("v IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___297 {
            actual___2668: std::sync::Arc<String>
        }
        impl ClosureGroup___297 {
            fn fn__13487(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2, 3), \")\").toString() == (v IN (1, 2, 3)) not ({})", self.actual___2668.clone()));
            }
        }
        let closure_group = ClosureGroup___297 {
            actual___2668: actual___2668.clone()
        };
        let fn__13487 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13487())
        };
        test___250.assert(t___13505, fn__13487.clone());
        let mut t___13507: SqlBuilder = SqlBuilder::new();
        t___13507.append_safe("v IN (");
        t___13507.append_int64_list(temper_core::ToListed::to_listed([1, 2]));
        t___13507.append_safe(")");
        let actual___2671: std::sync::Arc<String> = t___13507.accumulated().to_string();
        let mut t___13514: bool = Some(actual___2671.as_str()) == Some("v IN (1, 2)");
        #[derive(Clone)]
        struct ClosureGroup___298 {
            actual___2671: std::sync::Arc<String>
        }
        impl ClosureGroup___298 {
            fn fn__13486(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2), \")\").toString() == (v IN (1, 2)) not ({})", self.actual___2671.clone()));
            }
        }
        let closure_group = ClosureGroup___298 {
            actual___2671: actual___2671.clone()
        };
        let fn__13486 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13486())
        };
        test___250.assert(t___13514, fn__13486.clone());
        let mut t___13516: SqlBuilder = SqlBuilder::new();
        t___13516.append_safe("v IN (");
        t___13516.append_float64_list(temper_core::ToListed::to_listed([1.0f64, 2.0f64]));
        t___13516.append_safe(")");
        let actual___2674: std::sync::Arc<String> = t___13516.accumulated().to_string();
        let mut t___13523: bool = Some(actual___2674.as_str()) == Some("v IN (1.0, 2.0)");
        #[derive(Clone)]
        struct ClosureGroup___299 {
            actual___2674: std::sync::Arc<String>
        }
        impl ClosureGroup___299 {
            fn fn__13485(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1.0, 2.0), \")\").toString() == (v IN (1.0, 2.0)) not ({})", self.actual___2674.clone()));
            }
        }
        let closure_group = ClosureGroup___299 {
            actual___2674: actual___2674.clone()
        };
        let fn__13485 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13485())
        };
        test___250.assert(t___13523, fn__13485.clone());
        let mut t___13525: SqlBuilder = SqlBuilder::new();
        t___13525.append_safe("v IN (");
        t___13525.append_boolean_list(temper_core::ToListed::to_listed([true, false]));
        t___13525.append_safe(")");
        let actual___2677: std::sync::Arc<String> = t___13525.accumulated().to_string();
        let mut t___13532: bool = Some(actual___2677.as_str()) == Some("v IN (TRUE, FALSE)");
        #[derive(Clone)]
        struct ClosureGroup___300 {
            actual___2677: std::sync::Arc<String>
        }
        impl ClosureGroup___300 {
            fn fn__13484(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(true, false), \")\").toString() == (v IN (TRUE, FALSE)) not ({})", self.actual___2677.clone()));
            }
        }
        let closure_group = ClosureGroup___300 {
            actual___2677: actual___2677.clone()
        };
        let fn__13484 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13484())
        };
        test___250.assert(t___13532, fn__13484.clone());
        'ok___17510: {
            'orelse___2827: {
                t___6347 = match temper_std::temporal::Date::new(2024, 1, 1) {
                    Ok(x) => x,
                    _ => break 'orelse___2827
                };
                t___6348 = t___6347.clone();
                break 'ok___17510;
            }
            t___6348 = panic!();
        }
        'ok___17511: {
            'orelse___2828: {
                t___6349 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___2828
                };
                t___6350 = t___6349.clone();
                break 'ok___17511;
            }
            t___6350 = panic!();
        }
        let dates__2124: temper_core::List<temper_std::temporal::Date> = std::sync::Arc::new(vec![t___6348.clone(), t___6350.clone()]);
        let mut t___13534: SqlBuilder = SqlBuilder::new();
        t___13534.append_safe("v IN (");
        t___13534.append_date_list(temper_core::ToListed::to_listed(dates__2124.clone()));
        t___13534.append_safe(")");
        let actual___2680: std::sync::Arc<String> = t___13534.accumulated().to_string();
        let mut t___13541: bool = Some(actual___2680.as_str()) == Some("v IN ('2024-01-01', '2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___301 {
            actual___2680: std::sync::Arc<String>
        }
        impl ClosureGroup___301 {
            fn fn__13483(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, dates, \")\").toString() == (v IN ('2024-01-01', '2024-12-25')) not ({})", self.actual___2680.clone()));
            }
        }
        let closure_group = ClosureGroup___301 {
            actual___2680: actual___2680.clone()
        };
        let fn__13483 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13483())
        };
        test___250.assert(t___13541, fn__13483.clone());
        test___250.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_naNRendersAsNull__2683() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___251 = temper_std::testing::Test::new();
        let nan__2126: f64;
        nan__2126 = temper_core::float64::div(0.0f64, 0.0f64) ? ;
        let mut t___13475: SqlBuilder = SqlBuilder::new();
        t___13475.append_safe("v = ");
        t___13475.append_float64(nan__2126);
        let actual___2684: std::sync::Arc<String> = t___13475.accumulated().to_string();
        let mut t___13481: bool = Some(actual___2684.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___302 {
            actual___2684: std::sync::Arc<String>
        }
        impl ClosureGroup___302 {
            fn fn__13474(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, nan).toString() == (v = NULL) not ({})", self.actual___2684.clone()));
            }
        }
        let closure_group = ClosureGroup___302 {
            actual___2684: actual___2684.clone()
        };
        let fn__13474 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13474())
        };
        test___251.assert(t___13481, fn__13474.clone());
        test___251.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_infinityRendersAsNull__2687() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___252 = temper_std::testing::Test::new();
        let inf__2128: f64;
        inf__2128 = temper_core::float64::div(1.0f64, 0.0f64) ? ;
        let mut t___13466: SqlBuilder = SqlBuilder::new();
        t___13466.append_safe("v = ");
        t___13466.append_float64(inf__2128);
        let actual___2688: std::sync::Arc<String> = t___13466.accumulated().to_string();
        let mut t___13472: bool = Some(actual___2688.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___303 {
            actual___2688: std::sync::Arc<String>
        }
        impl ClosureGroup___303 {
            fn fn__13465(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, inf).toString() == (v = NULL) not ({})", self.actual___2688.clone()));
            }
        }
        let closure_group = ClosureGroup___303 {
            actual___2688: actual___2688.clone()
        };
        let fn__13465 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13465())
        };
        test___252.assert(t___13472, fn__13465.clone());
        test___252.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_negativeInfinityRendersAsNull__2691() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___253 = temper_std::testing::Test::new();
        let ninf__2130: f64;
        ninf__2130 = temper_core::float64::div(-1.0f64, 0.0f64) ? ;
        let mut t___13457: SqlBuilder = SqlBuilder::new();
        t___13457.append_safe("v = ");
        t___13457.append_float64(ninf__2130);
        let actual___2692: std::sync::Arc<String> = t___13457.accumulated().to_string();
        let mut t___13463: bool = Some(actual___2692.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___304 {
            actual___2692: std::sync::Arc<String>
        }
        impl ClosureGroup___304 {
            fn fn__13456(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, ninf).toString() == (v = NULL) not ({})", self.actual___2692.clone()));
            }
        }
        let closure_group = ClosureGroup___304 {
            actual___2692: actual___2692.clone()
        };
        let fn__13456 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13456())
        };
        test___253.assert(t___13463, fn__13456.clone());
        test___253.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_normalValuesStillWork__2695() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___254 = temper_std::testing::Test::new();
        let mut t___13432: SqlBuilder = SqlBuilder::new();
        t___13432.append_safe("v = ");
        t___13432.append_float64(3.14f64);
        let actual___2696: std::sync::Arc<String> = t___13432.accumulated().to_string();
        let mut t___13438: bool = Some(actual___2696.as_str()) == Some("v = 3.14");
        #[derive(Clone)]
        struct ClosureGroup___305 {
            actual___2696: std::sync::Arc<String>
        }
        impl ClosureGroup___305 {
            fn fn__13431(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 3.14).toString() == (v = 3.14) not ({})", self.actual___2696.clone()));
            }
        }
        let closure_group = ClosureGroup___305 {
            actual___2696: actual___2696.clone()
        };
        let fn__13431 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13431())
        };
        test___254.assert(t___13438, fn__13431.clone());
        let mut t___13440: SqlBuilder = SqlBuilder::new();
        t___13440.append_safe("v = ");
        t___13440.append_float64(0.0f64);
        let actual___2699: std::sync::Arc<String> = t___13440.accumulated().to_string();
        let mut t___13446: bool = Some(actual___2699.as_str()) == Some("v = 0.0");
        #[derive(Clone)]
        struct ClosureGroup___306 {
            actual___2699: std::sync::Arc<String>
        }
        impl ClosureGroup___306 {
            fn fn__13430(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 0.0).toString() == (v = 0.0) not ({})", self.actual___2699.clone()));
            }
        }
        let closure_group = ClosureGroup___306 {
            actual___2699: actual___2699.clone()
        };
        let fn__13430 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13430())
        };
        test___254.assert(t___13446, fn__13430.clone());
        let mut t___13448: SqlBuilder = SqlBuilder::new();
        t___13448.append_safe("v = ");
        t___13448.append_float64(-42.5f64);
        let actual___2702: std::sync::Arc<String> = t___13448.accumulated().to_string();
        let mut t___13454: bool = Some(actual___2702.as_str()) == Some("v = -42.5");
        #[derive(Clone)]
        struct ClosureGroup___307 {
            actual___2702: std::sync::Arc<String>
        }
        impl ClosureGroup___307 {
            fn fn__13429(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, -42.5).toString() == (v = -42.5) not ({})", self.actual___2702.clone()));
            }
        }
        let closure_group = ClosureGroup___307 {
            actual___2702: actual___2702.clone()
        };
        let fn__13429 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13429())
        };
        test___254.assert(t___13454, fn__13429.clone());
        test___254.soft_fail_to_hard()
    }
    #[test]
    fn sqlDateRendersWithQuotes__2705() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___255 = temper_std::testing::Test::new();
        let mut t___6243: temper_std::temporal::Date;
        let d__2133: temper_std::temporal::Date;
        'ok___17512: {
            'orelse___2829: {
                t___6243 = match temper_std::temporal::Date::new(2024, 6, 15) {
                    Ok(x) => x,
                    _ => break 'orelse___2829
                };
                d__2133 = t___6243.clone();
                break 'ok___17512;
            }
            d__2133 = panic!();
        }
        let mut t___13421: SqlBuilder = SqlBuilder::new();
        t___13421.append_safe("v = ");
        t___13421.append_date(d__2133.clone());
        let actual___2706: std::sync::Arc<String> = t___13421.accumulated().to_string();
        let mut t___13427: bool = Some(actual___2706.as_str()) == Some("v = '2024-06-15'");
        #[derive(Clone)]
        struct ClosureGroup___308 {
            actual___2706: std::sync::Arc<String>
        }
        impl ClosureGroup___308 {
            fn fn__13420(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, d).toString() == (v = '2024-06-15') not ({})", self.actual___2706.clone()));
            }
        }
        let closure_group = ClosureGroup___308 {
            actual___2706: actual___2706.clone()
        };
        let fn__13420 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13420())
        };
        test___255.assert(t___13427, fn__13420.clone());
        test___255.soft_fail_to_hard()
    }
    #[test]
    fn nesting__2709() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___256 = temper_std::testing::Test::new();
        let name__2135: std::sync::Arc<String> = std::sync::Arc::new("Someone".to_string());
        let mut t___13389: SqlBuilder = SqlBuilder::new();
        t___13389.append_safe("where p.last_name = ");
        t___13389.append_string("Someone");
        let condition__2136: SqlFragment = t___13389.accumulated();
        let mut t___13393: SqlBuilder = SqlBuilder::new();
        t___13393.append_safe("select p.id from person p ");
        t___13393.append_fragment(condition__2136.clone());
        let actual___2711: std::sync::Arc<String> = t___13393.accumulated().to_string();
        let mut t___13399: bool = Some(actual___2711.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___309 {
            actual___2711: std::sync::Arc<String>
        }
        impl ClosureGroup___309 {
            fn fn__13388(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___2711.clone()));
            }
        }
        let closure_group = ClosureGroup___309 {
            actual___2711: actual___2711.clone()
        };
        let fn__13388 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13388())
        };
        test___256.assert(t___13399, fn__13388.clone());
        let mut t___13401: SqlBuilder = SqlBuilder::new();
        t___13401.append_safe("select p.id from person p ");
        t___13401.append_part(SqlPart::new(condition__2136.to_source()));
        let actual___2714: std::sync::Arc<String> = t___13401.accumulated().to_string();
        let mut t___13408: bool = Some(actual___2714.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___310 {
            actual___2714: std::sync::Arc<String>
        }
        impl ClosureGroup___310 {
            fn fn__13387(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition.toSource()).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___2714.clone()));
            }
        }
        let closure_group = ClosureGroup___310 {
            actual___2714: actual___2714.clone()
        };
        let fn__13387 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13387())
        };
        test___256.assert(t___13408, fn__13387.clone());
        let parts__2137: temper_core::List<SqlPart> = std::sync::Arc::new(vec![SqlPart::new(SqlString::new("a'b")), SqlPart::new(SqlInt32::new(3))]);
        let mut t___13412: SqlBuilder = SqlBuilder::new();
        t___13412.append_safe("select ");
        t___13412.append_part_list(parts__2137.clone());
        let actual___2717: std::sync::Arc<String> = t___13412.accumulated().to_string();
        let mut t___13418: bool = Some(actual___2717.as_str()) == Some("select 'a''b', 3");
        #[derive(Clone)]
        struct ClosureGroup___311 {
            actual___2717: std::sync::Arc<String>
        }
        impl ClosureGroup___311 {
            fn fn__13386(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, parts).toString() == (select 'a''b', 3) not ({})", self.actual___2717.clone()));
            }
        }
        let closure_group = ClosureGroup___311 {
            actual___2717: actual___2717.clone()
        };
        let fn__13386 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13386())
        };
        test___256.assert(t___13418, fn__13386.clone());
        test___256.soft_fail_to_hard()
    }
    #[test]
    fn sqlInt32_negativeAndZeroValues__2720() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___257 = temper_std::testing::Test::new();
        let mut t___13370: SqlBuilder = SqlBuilder::new();
        t___13370.append_safe("v = ");
        t___13370.append_int32(-42);
        let mut t___13376: bool = Some(t___13370.accumulated().to_string().as_str()) == Some("v = -42");
        #[derive(Clone)]
        struct ClosureGroup___312 {}
        impl ClosureGroup___312 {
            fn fn__13369(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative int".to_string());
            }
        }
        let closure_group = ClosureGroup___312 {};
        let fn__13369 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13369())
        };
        test___257.assert(t___13376, fn__13369.clone());
        let mut t___13378: SqlBuilder = SqlBuilder::new();
        t___13378.append_safe("v = ");
        t___13378.append_int32(0);
        let mut t___13384: bool = Some(t___13378.accumulated().to_string().as_str()) == Some("v = 0");
        #[derive(Clone)]
        struct ClosureGroup___313 {}
        impl ClosureGroup___313 {
            fn fn__13368(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("zero int".to_string());
            }
        }
        let closure_group = ClosureGroup___313 {};
        let fn__13368 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13368())
        };
        test___257.assert(t___13384, fn__13368.clone());
        test___257.soft_fail_to_hard()
    }
    #[test]
    fn sqlInt64_negativeValue__2723() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___258 = temper_std::testing::Test::new();
        let mut t___13360: SqlBuilder = SqlBuilder::new();
        t___13360.append_safe("v = ");
        t___13360.append_int64(-99);
        let mut t___13366: bool = Some(t___13360.accumulated().to_string().as_str()) == Some("v = -99");
        #[derive(Clone)]
        struct ClosureGroup___314 {}
        impl ClosureGroup___314 {
            fn fn__13359(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative int64".to_string());
            }
        }
        let closure_group = ClosureGroup___314 {};
        let fn__13359 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13359())
        };
        test___258.assert(t___13366, fn__13359.clone());
        test___258.soft_fail_to_hard()
    }
    #[test]
    fn singleElementListRendering__2725() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___259 = temper_std::testing::Test::new();
        let mut t___13341: SqlBuilder = SqlBuilder::new();
        t___13341.append_safe("v IN (");
        t___13341.append_int32_list(temper_core::ToListed::to_listed([42]));
        t___13341.append_safe(")");
        let mut t___13348: bool = Some(t___13341.accumulated().to_string().as_str()) == Some("v IN (42)");
        #[derive(Clone)]
        struct ClosureGroup___315 {}
        impl ClosureGroup___315 {
            fn fn__13340(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("single int".to_string());
            }
        }
        let closure_group = ClosureGroup___315 {};
        let fn__13340 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13340())
        };
        test___259.assert(t___13348, fn__13340.clone());
        let mut t___13350: SqlBuilder = SqlBuilder::new();
        t___13350.append_safe("v IN (");
        t___13350.append_string_list(temper_core::ToListed::to_listed([std::sync::Arc::new("only".to_string())]));
        t___13350.append_safe(")");
        let mut t___13357: bool = Some(t___13350.accumulated().to_string().as_str()) == Some("v IN ('only')");
        #[derive(Clone)]
        struct ClosureGroup___316 {}
        impl ClosureGroup___316 {
            fn fn__13339(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("single string".to_string());
            }
        }
        let closure_group = ClosureGroup___316 {};
        let fn__13339 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13339())
        };
        test___259.assert(t___13357, fn__13339.clone());
        test___259.soft_fail_to_hard()
    }
    #[test]
    fn sqlDefaultRendersDefaultKeyword__2728() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___260 = temper_std::testing::Test::new();
        let b__2142: SqlBuilder = SqlBuilder::new();
        b__2142.append_safe("v = ");
        b__2142.append_part(SqlPart::new(SqlDefault::new()));
        let mut t___13337: bool = Some(b__2142.accumulated().to_string().as_str()) == Some("v = DEFAULT");
        #[derive(Clone)]
        struct ClosureGroup___317 {}
        impl ClosureGroup___317 {
            fn fn__13329(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("default keyword".to_string());
            }
        }
        let closure_group = ClosureGroup___317 {};
        let fn__13329 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13329())
        };
        test___260.assert(t___13337, fn__13329.clone());
        test___260.soft_fail_to_hard()
    }
    #[test]
    fn sqlStringWithBackslash__2729() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___261 = temper_std::testing::Test::new();
        let mut t___13321: SqlBuilder = SqlBuilder::new();
        t___13321.append_safe("v = ");
        t___13321.append_string("a\\b");
        let mut t___13327: bool = Some(t___13321.accumulated().to_string().as_str()) == Some("v = 'a\\b'");
        #[derive(Clone)]
        struct ClosureGroup___318 {}
        impl ClosureGroup___318 {
            fn fn__13320(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("backslash passthrough".to_string());
            }
        }
        let closure_group = ClosureGroup___318 {};
        let fn__13320 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13320())
        };
        test___261.assert(t___13327, fn__13320.clone());
        test___261.soft_fail_to_hard()
    }
    use super::*;
}
