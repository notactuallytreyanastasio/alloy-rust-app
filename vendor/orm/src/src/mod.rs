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
    pub fn new(field__554: impl temper_core::ToArcString, message__555: impl temper_core::ToArcString) -> ChangesetError {
        let field__554 = field__554.to_arc_string();
        let message__555 = message__555.to_arc_string();
        let field;
        let message;
        field = field__554.clone();
        message = message__555.clone();
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
    fn cast(& self, allowedFields__565: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_required(& self, fields__568: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_length(& self, field__571: SafeIdentifier, min__572: i32, max__573: i32) -> Changeset;
    fn validate_int(& self, field__576: SafeIdentifier) -> Changeset;
    fn validate_int64(& self, field__579: SafeIdentifier) -> Changeset;
    fn validate_float(& self, field__582: SafeIdentifier) -> Changeset;
    fn validate_bool(& self, field__585: SafeIdentifier) -> Changeset;
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment>;
    fn to_update_sql(& self, id__590: i32) -> temper_core::Result<SqlFragment>;
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
    pub fn cast(& self, allowedFields__606: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let allowedFields__606 = allowedFields__606.to_list();
        let mb__608: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        #[derive(Clone)]
        struct ClosureGroup___1 {
            this__187: ChangesetImpl, mb__608: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>>
        }
        impl ClosureGroup___1 {
            fn fn__11363(& self, f__609: SafeIdentifier) {
                let mut t___11361: std::sync::Arc<String>;
                let mut t___11358: std::sync::Arc<String> = f__609.sql_value();
                let val__610: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.this__187.0.params, t___11358.clone(), std::sync::Arc::new("".to_string()));
                if ! val__610.is_empty() {
                    t___11361 = f__609.sql_value();
                    temper_core::MapBuilder::set( & self.mb__608, t___11361.clone(), val__610.clone());
                }
            }
        }
        let closure_group = ClosureGroup___1 {
            this__187: self.clone(), mb__608: mb__608.clone()
        };
        let fn__11363 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__609: SafeIdentifier | closure_group.fn__11363(f__609))
        };
        temper_core::listed::list_for_each( & allowedFields__606, & ( * fn__11363.clone()));
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__608), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn validate_required(& self, fields__612: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let fields__612 = fields__612.to_list();
        let return__324: Changeset;
        let mut t___11356: temper_core::List<ChangesetError>;
        let mut t___6537: TableDef;
        let mut t___6538: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___6539: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__613: {
            if ! self.0.is_valid {
                return__324 = Changeset::new(self.clone());
                break 'fn__613;
            }
            let eb__614: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
            let mut valid__615: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___2 {
                this__188: ChangesetImpl, eb__614: temper_core::ListBuilder<ChangesetError>, valid__615: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___2 {
                fn fn__11352(& self, f__616: SafeIdentifier) {
                    let mut t___11350: ChangesetError;
                    let mut t___11347: std::sync::Arc<String> = f__616.sql_value();
                    if ! temper_core::MappedTrait::has( & self.this__188.0.changes, t___11347.clone()) {
                        t___11350 = ChangesetError::new(f__616.sql_value(), "is required");
                        temper_core::listed::add( & self.eb__614, t___11350.clone(), None);
                        {
                            * self.valid__615.write().unwrap() = false;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___2 {
                this__188: self.clone(), eb__614: eb__614.clone(), valid__615: valid__615.clone()
            };
            let fn__11352 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__616: SafeIdentifier | closure_group.fn__11352(f__616))
            };
            temper_core::listed::list_for_each( & fields__612, & ( * fn__11352.clone()));
            t___6537 = self.0.table_def.clone();
            t___6538 = self.0.params.clone();
            t___6539 = self.0.changes.clone();
            t___11356 = temper_core::ListedTrait::to_list( & eb__614);
            return__324 = Changeset::new(ChangesetImpl::new(t___6537.clone(), t___6538.clone(), t___6539.clone(), t___11356.clone(), temper_core::read_locked( & valid__615)));
        }
        return return__324.clone();
    }
    pub fn validate_length(& self, field__618: SafeIdentifier, min__619: i32, max__620: i32) -> Changeset {
        let return__325: Changeset;
        let mut t___11334: std::sync::Arc<String>;
        let mut t___11345: temper_core::List<ChangesetError>;
        let mut t___6520: bool;
        let mut t___6526: TableDef;
        let mut t___6527: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___6528: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__621: {
            if ! self.0.is_valid {
                return__325 = Changeset::new(self.clone());
                break 'fn__621;
            }
            t___11334 = field__618.sql_value();
            let val__622: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___11334.clone(), std::sync::Arc::new("".to_string()));
            let len__623: i32 = temper_core::string::count_between( & val__622, 0usize, val__622.len());
            if Some(len__623) < Some(min__619) {
                t___6520 = true;
            } else {
                t___6520 = Some(len__623) > Some(max__620);
            }
            if t___6520 {
                let msg__624: std::sync::Arc<String> = std::sync::Arc::new(format!("must be between {} and {} characters", min__619, max__620));
                let eb__625: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__625, ChangesetError::new(field__618.sql_value(), msg__624.clone()), None);
                t___6526 = self.0.table_def.clone();
                t___6527 = self.0.params.clone();
                t___6528 = self.0.changes.clone();
                t___11345 = temper_core::ListedTrait::to_list( & eb__625);
                return__325 = Changeset::new(ChangesetImpl::new(t___6526.clone(), t___6527.clone(), t___6528.clone(), t___11345.clone(), false));
                break 'fn__621;
            }
            return__325 = Changeset::new(self.clone());
        }
        return return__325.clone();
    }
    pub fn validate_int(& self, field__627: SafeIdentifier) -> Changeset {
        let return__326: Changeset;
        let mut t___11325: std::sync::Arc<String>;
        let mut t___11332: temper_core::List<ChangesetError>;
        let mut t___6511: TableDef;
        let mut t___6512: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___6513: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__628: {
            if ! self.0.is_valid {
                return__326 = Changeset::new(self.clone());
                break 'fn__628;
            }
            t___11325 = field__627.sql_value();
            let val__629: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___11325.clone(), std::sync::Arc::new("".to_string()));
            if val__629.is_empty() {
                return__326 = Changeset::new(self.clone());
                break 'fn__628;
            }
            let parseOk__630: bool;
            'ok___11468: {
                'orelse___2009: {
                    match temper_core::string::to_int( & val__629, None) {
                        Ok(x) => x,
                        _ => break 'orelse___2009
                    };
                    parseOk__630 = true;
                    break 'ok___11468;
                }
                parseOk__630 = false;
            }
            if ! parseOk__630 {
                let eb__631: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__631, ChangesetError::new(field__627.sql_value(), "must be an integer"), None);
                t___6511 = self.0.table_def.clone();
                t___6512 = self.0.params.clone();
                t___6513 = self.0.changes.clone();
                t___11332 = temper_core::ListedTrait::to_list( & eb__631);
                return__326 = Changeset::new(ChangesetImpl::new(t___6511.clone(), t___6512.clone(), t___6513.clone(), t___11332.clone(), false));
                break 'fn__628;
            }
            return__326 = Changeset::new(self.clone());
        }
        return return__326.clone();
    }
    pub fn validate_int64(& self, field__633: SafeIdentifier) -> Changeset {
        let return__327: Changeset;
        let mut t___11316: std::sync::Arc<String>;
        let mut t___11323: temper_core::List<ChangesetError>;
        let mut t___6498: TableDef;
        let mut t___6499: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___6500: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__634: {
            if ! self.0.is_valid {
                return__327 = Changeset::new(self.clone());
                break 'fn__634;
            }
            t___11316 = field__633.sql_value();
            let val__635: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___11316.clone(), std::sync::Arc::new("".to_string()));
            if val__635.is_empty() {
                return__327 = Changeset::new(self.clone());
                break 'fn__634;
            }
            let parseOk__636: bool;
            'ok___11470: {
                'orelse___2010: {
                    match temper_core::string::to_int64( & val__635, None) {
                        Ok(x) => x,
                        _ => break 'orelse___2010
                    };
                    parseOk__636 = true;
                    break 'ok___11470;
                }
                parseOk__636 = false;
            }
            if ! parseOk__636 {
                let eb__637: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__637, ChangesetError::new(field__633.sql_value(), "must be a 64-bit integer"), None);
                t___6498 = self.0.table_def.clone();
                t___6499 = self.0.params.clone();
                t___6500 = self.0.changes.clone();
                t___11323 = temper_core::ListedTrait::to_list( & eb__637);
                return__327 = Changeset::new(ChangesetImpl::new(t___6498.clone(), t___6499.clone(), t___6500.clone(), t___11323.clone(), false));
                break 'fn__634;
            }
            return__327 = Changeset::new(self.clone());
        }
        return return__327.clone();
    }
    pub fn validate_float(& self, field__639: SafeIdentifier) -> Changeset {
        let return__328: Changeset;
        let mut t___11307: std::sync::Arc<String>;
        let mut t___11314: temper_core::List<ChangesetError>;
        let mut t___6485: TableDef;
        let mut t___6486: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___6487: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__640: {
            if ! self.0.is_valid {
                return__328 = Changeset::new(self.clone());
                break 'fn__640;
            }
            t___11307 = field__639.sql_value();
            let val__641: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___11307.clone(), std::sync::Arc::new("".to_string()));
            if val__641.is_empty() {
                return__328 = Changeset::new(self.clone());
                break 'fn__640;
            }
            let parseOk__642: bool;
            'ok___11472: {
                'orelse___2011: {
                    match temper_core::string::to_float64( & val__641) {
                        Ok(x) => x,
                        _ => break 'orelse___2011
                    };
                    parseOk__642 = true;
                    break 'ok___11472;
                }
                parseOk__642 = false;
            }
            if ! parseOk__642 {
                let eb__643: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__643, ChangesetError::new(field__639.sql_value(), "must be a number"), None);
                t___6485 = self.0.table_def.clone();
                t___6486 = self.0.params.clone();
                t___6487 = self.0.changes.clone();
                t___11314 = temper_core::ListedTrait::to_list( & eb__643);
                return__328 = Changeset::new(ChangesetImpl::new(t___6485.clone(), t___6486.clone(), t___6487.clone(), t___11314.clone(), false));
                break 'fn__640;
            }
            return__328 = Changeset::new(self.clone());
        }
        return return__328.clone();
    }
    pub fn validate_bool(& self, field__645: SafeIdentifier) -> Changeset {
        let return__329: Changeset;
        let mut t___11298: std::sync::Arc<String>;
        let mut t___11305: temper_core::List<ChangesetError>;
        let mut t___6460: bool;
        let mut t___6461: bool;
        let mut t___6463: bool;
        let mut t___6464: bool;
        let mut t___6466: bool;
        let mut t___6472: TableDef;
        let mut t___6473: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___6474: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__646: {
            if ! self.0.is_valid {
                return__329 = Changeset::new(self.clone());
                break 'fn__646;
            }
            t___11298 = field__645.sql_value();
            let val__647: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___11298.clone(), std::sync::Arc::new("".to_string()));
            if val__647.is_empty() {
                return__329 = Changeset::new(self.clone());
                break 'fn__646;
            }
            let isTrue__648: bool;
            if Some(val__647.as_str()) == Some("true") {
                isTrue__648 = true;
            } else {
                if Some(val__647.as_str()) == Some("1") {
                    t___6461 = true;
                } else {
                    if Some(val__647.as_str()) == Some("yes") {
                        t___6460 = true;
                    } else {
                        t___6460 = Some(val__647.as_str()) == Some("on");
                    }
                    t___6461 = t___6460;
                }
                isTrue__648 = t___6461;
            }
            let isFalse__649: bool;
            if Some(val__647.as_str()) == Some("false") {
                isFalse__649 = true;
            } else {
                if Some(val__647.as_str()) == Some("0") {
                    t___6464 = true;
                } else {
                    if Some(val__647.as_str()) == Some("no") {
                        t___6463 = true;
                    } else {
                        t___6463 = Some(val__647.as_str()) == Some("off");
                    }
                    t___6464 = t___6463;
                }
                isFalse__649 = t___6464;
            }
            if ! isTrue__648 {
                t___6466 = ! isFalse__649;
            } else {
                t___6466 = false;
            }
            if t___6466 {
                let eb__650: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__650, ChangesetError::new(field__645.sql_value(), "must be a boolean (true/false/1/0/yes/no/on/off)"), None);
                t___6472 = self.0.table_def.clone();
                t___6473 = self.0.params.clone();
                t___6474 = self.0.changes.clone();
                t___11305 = temper_core::ListedTrait::to_list( & eb__650);
                return__329 = Changeset::new(ChangesetImpl::new(t___6472.clone(), t___6473.clone(), t___6474.clone(), t___11305.clone(), false));
                break 'fn__646;
            }
            return__329 = Changeset::new(self.clone());
        }
        return return__329.clone();
    }
    fn parse_bool_sql_part(& self, val__652: impl temper_core::ToArcString) -> temper_core::Result<SqlBoolean> {
        let val__652 = val__652.to_arc_string();
        let return__330: SqlBoolean;
        let mut t___6449: bool;
        let mut t___6450: bool;
        let mut t___6451: bool;
        let mut t___6453: bool;
        let mut t___6454: bool;
        let mut t___6455: bool;
        'fn__653: {
            if Some(val__652.as_str()) == Some("true") {
                t___6451 = true;
            } else {
                if Some(val__652.as_str()) == Some("1") {
                    t___6450 = true;
                } else {
                    if Some(val__652.as_str()) == Some("yes") {
                        t___6449 = true;
                    } else {
                        t___6449 = Some(val__652.as_str()) == Some("on");
                    }
                    t___6450 = t___6449;
                }
                t___6451 = t___6450;
            }
            if t___6451 {
                return__330 = SqlBoolean::new(true);
                break 'fn__653;
            }
            if Some(val__652.as_str()) == Some("false") {
                t___6455 = true;
            } else {
                if Some(val__652.as_str()) == Some("0") {
                    t___6454 = true;
                } else {
                    if Some(val__652.as_str()) == Some("no") {
                        t___6453 = true;
                    } else {
                        t___6453 = Some(val__652.as_str()) == Some("off");
                    }
                    t___6454 = t___6453;
                }
                t___6455 = t___6454;
            }
            if t___6455 {
                return__330 = SqlBoolean::new(false);
                break 'fn__653;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__330.clone());
    }
    fn value_to_sql_part(& self, fieldDef__655: FieldDef, val__656: impl temper_core::ToArcString) -> temper_core::Result<SqlPart> {
        let val__656 = val__656.to_arc_string();
        let return__331: SqlPart;
        let mut t___6436: i32;
        let mut t___6439: i64;
        let mut t___6442: f64;
        let mut t___6447: temper_std::temporal::Date;
        'fn__657: {
            let ft__658: FieldType = fieldDef__655.field_type();
            if temper_core::is::<StringField>(ft__658.clone()) {
                return__331 = SqlPart::new(SqlString::new(val__656.clone()));
                break 'fn__657;
            }
            if temper_core::is::<IntField>(ft__658.clone()) {
                t___6436 = temper_core::string::to_int( & val__656, None) ? ;
                return__331 = SqlPart::new(SqlInt32::new(t___6436));
                break 'fn__657;
            }
            if temper_core::is::<Int64Field>(ft__658.clone()) {
                t___6439 = temper_core::string::to_int64( & val__656, None) ? ;
                return__331 = SqlPart::new(SqlInt64::new(t___6439));
                break 'fn__657;
            }
            if temper_core::is::<FloatField>(ft__658.clone()) {
                t___6442 = temper_core::string::to_float64( & val__656) ? ;
                return__331 = SqlPart::new(SqlFloat64::new(t___6442));
                break 'fn__657;
            }
            if temper_core::is::<BoolField>(ft__658.clone()) {
                return__331 = SqlPart::new(self.parse_bool_sql_part(val__656.clone()) ? );
                break 'fn__657;
            }
            if temper_core::is::<DateField>(ft__658.clone()) {
                t___6447 = temper_std::temporal::Date::from_iso_string(val__656.clone()) ? ;
                return__331 = SqlPart::new(SqlDate::new(t___6447.clone()));
                break 'fn__657;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__331.clone());
    }
    pub fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___11246: i32;
        let mut t___11251: std::sync::Arc<String>;
        let mut t___11252: bool;
        let mut t___11257: i32;
        let mut t___11259: std::sync::Arc<String>;
        let mut t___11263: std::sync::Arc<String>;
        let mut t___11278: i32;
        let mut t___6400: bool;
        let mut t___6408: FieldDef;
        let mut t___6413: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let mut i__661: i32 = 0;
        'loop___11548: loop {
            t___11246 = temper_core::ListedTrait::len( & self.0.table_def.fields());
            if ! (Some(i__661) < Some(t___11246)) {
                break;
            }
            let f__662: FieldDef = temper_core::ListedTrait::get( & self.0.table_def.fields(), i__661);
            if ! f__662.nullable() {
                t___11251 = f__662.name().sql_value();
                t___11252 = temper_core::MappedTrait::has( & self.0.changes, t___11251.clone());
                t___6400 = ! t___11252;
            } else {
                t___6400 = false;
            }
            if t___6400 {
                return Err(temper_core::Error::new());
            }
            i__661 = i__661.wrapping_add(1);
        }
        let pairs__663: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__663)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let colNames__664: temper_core::ListBuilder<std::sync::Arc<String>> = temper_core::listed::new_builder();
        let valParts__665: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        let mut i__666: i32 = 0;
        'loop___11549: loop {
            t___11257 = temper_core::ListedTrait::len( & pairs__663);
            if ! (Some(i__666) < Some(t___11257)) {
                break;
            }
            let pair__667: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__663, i__666);
            t___11259 = pair__667.key();
            t___6408 = self.0.table_def.field(t___11259.clone()) ? ;
            let fd__668: FieldDef = t___6408.clone();
            temper_core::listed::add( & colNames__664, fd__668.name().sql_value(), None);
            t___11263 = pair__667.value();
            t___6413 = self.value_to_sql_part(fd__668.clone(), t___11263.clone()) ? ;
            temper_core::listed::add( & valParts__665, t___6413.clone(), None);
            i__666 = i__666.wrapping_add(1);
        }
        let b__669: SqlBuilder = SqlBuilder::new();
        b__669.append_safe("INSERT INTO ");
        b__669.append_safe(self.0.table_def.table_name().sql_value());
        b__669.append_safe(" (");
        let mut t___11271: temper_core::List<std::sync::Arc<String>> = temper_core::ListedTrait::to_list( & colNames__664);
        #[derive(Clone)]
        struct ClosureGroup___3 {}
        impl ClosureGroup___3 {
            fn fn__11244(& self, c__670: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let c__670 = c__670.to_arc_string();
                return c__670.clone();
            }
        }
        let closure_group = ClosureGroup___3 {};
        let fn__11244 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__670: std::sync::Arc<String> | closure_group.fn__11244(c__670))
        };
        b__669.append_safe(temper_core::listed::join( & t___11271, std::sync::Arc::new(", ".to_string()), & ( * fn__11244.clone())));
        b__669.append_safe(") VALUES (");
        b__669.append_part(temper_core::ListedTrait::get( & valParts__665, 0));
        let mut j__671: i32 = 1;
        'loop___11550: loop {
            t___11278 = temper_core::ListedTrait::len( & valParts__665);
            if ! (Some(j__671) < Some(t___11278)) {
                break;
            }
            b__669.append_safe(", ");
            b__669.append_part(temper_core::ListedTrait::get( & valParts__665, j__671));
            j__671 = j__671.wrapping_add(1);
        }
        b__669.append_safe(")");
        return Ok(b__669.accumulated());
    }
    pub fn to_update_sql(& self, id__673: i32) -> temper_core::Result<SqlFragment> {
        let mut t___11231: i32;
        let mut t___11234: std::sync::Arc<String>;
        let mut t___11239: std::sync::Arc<String>;
        let mut t___6381: FieldDef;
        let mut t___6387: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let pairs__675: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__675)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let b__676: SqlBuilder = SqlBuilder::new();
        b__676.append_safe("UPDATE ");
        b__676.append_safe(self.0.table_def.table_name().sql_value());
        b__676.append_safe(" SET ");
        let mut i__677: i32 = 0;
        'loop___11551: loop {
            t___11231 = temper_core::ListedTrait::len( & pairs__675);
            if ! (Some(i__677) < Some(t___11231)) {
                break;
            }
            if Some(i__677) > Some(0) {
                b__676.append_safe(", ");
            }
            let pair__678: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__675, i__677);
            t___11234 = pair__678.key();
            t___6381 = self.0.table_def.field(t___11234.clone()) ? ;
            let fd__679: FieldDef = t___6381.clone();
            b__676.append_safe(fd__679.name().sql_value());
            b__676.append_safe(" = ");
            t___11239 = pair__678.value();
            t___6387 = self.value_to_sql_part(fd__679.clone(), t___11239.clone()) ? ;
            b__676.append_part(t___6387.clone());
            i__677 = i__677.wrapping_add(1);
        }
        b__676.append_safe(" WHERE id = ");
        b__676.append_int32(id__673);
        return Ok(b__676.accumulated());
    }
    pub fn new(_tableDef__681: TableDef, _params__682: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _changes__683: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _errors__684: impl temper_core::ToList<ChangesetError>, _isValid__685: bool) -> ChangesetImpl {
        let _errors__684 = _errors__684.to_list();
        let table_def;
        let params;
        let changes;
        let errors;
        let is_valid;
        table_def = _tableDef__681.clone();
        params = _params__682.clone();
        changes = _changes__683.clone();
        errors = _errors__684.clone();
        is_valid = _isValid__685;
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
    fn cast(& self, allowedFields__606: temper_core::List<SafeIdentifier>) -> Changeset {
        self.cast(allowedFields__606)
    }
    fn validate_required(& self, fields__612: temper_core::List<SafeIdentifier>) -> Changeset {
        self.validate_required(fields__612)
    }
    fn validate_length(& self, field__618: SafeIdentifier, min__619: i32, max__620: i32) -> Changeset {
        self.validate_length(field__618, min__619, max__620)
    }
    fn validate_int(& self, field__627: SafeIdentifier) -> Changeset {
        self.validate_int(field__627)
    }
    fn validate_int64(& self, field__633: SafeIdentifier) -> Changeset {
        self.validate_int64(field__633)
    }
    fn validate_float(& self, field__639: SafeIdentifier) -> Changeset {
        self.validate_float(field__639)
    }
    fn validate_bool(& self, field__645: SafeIdentifier) -> Changeset {
        self.validate_bool(field__645)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        self.to_insert_sql()
    }
    fn to_update_sql(& self, id__673: i32) -> temper_core::Result<SqlFragment> {
        self.to_update_sql(id__673)
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
    pub fn new(joinType__801: JoinType, table__802: SafeIdentifier, onCondition__803: Option<SqlFragment>) -> JoinClause {
        let join_type;
        let table;
        let on_condition;
        join_type = joinType__801.clone();
        table = table__802.clone();
        on_condition = onCondition__803.clone();
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
    pub fn new(field__816: SafeIdentifier, ascending__817: bool, nullsPos__818: Option<NullsPosition>) -> OrderClause {
        let field;
        let ascending;
        let nulls_pos;
        field = field__816.clone();
        ascending = ascending__817;
        nulls_pos = nullsPos__818.clone();
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
    pub fn new(_condition__837: SqlFragment) -> AndCondition {
        let condition;
        condition = _condition__837.clone();
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
    pub fn new(_condition__844: SqlFragment) -> OrCondition {
        let condition;
        condition = _condition__844.clone();
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
    pub fn r#where(& self, condition__858: SqlFragment) -> Query {
        let nb__860: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__860, WhereClause::new(AndCondition::new(condition__858.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__860), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn or_where(& self, condition__862: SqlFragment) -> Query {
        let nb__864: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__864, WhereClause::new(OrCondition::new(condition__862.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__864), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn where_null(& self, field__866: SafeIdentifier) -> Query {
        let b__868: SqlBuilder = SqlBuilder::new();
        b__868.append_safe(field__866.sql_value());
        b__868.append_safe(" IS NULL");
        let mut t___10830: SqlFragment = b__868.accumulated();
        return self.r#where(t___10830.clone());
    }
    pub fn where_not_null(& self, field__870: SafeIdentifier) -> Query {
        let b__872: SqlBuilder = SqlBuilder::new();
        b__872.append_safe(field__870.sql_value());
        b__872.append_safe(" IS NOT NULL");
        let mut t___10824: SqlFragment = b__872.accumulated();
        return self.r#where(t___10824.clone());
    }
    pub fn where_in(& self, field__874: SafeIdentifier, values__875: impl temper_core::ToList<SqlPart>) -> Query {
        let values__875 = values__875.to_list();
        let return__403: Query;
        let mut t___10805: SqlFragment;
        let mut t___10813: i32;
        let mut t___10818: SqlFragment;
        'fn__876: {
            if temper_core::ListedTrait::is_empty( & values__875) {
                let b__877: SqlBuilder = SqlBuilder::new();
                b__877.append_safe("1 = 0");
                t___10805 = b__877.accumulated();
                return__403 = self.r#where(t___10805.clone());
                break 'fn__876;
            }
            let b__878: SqlBuilder = SqlBuilder::new();
            b__878.append_safe(field__874.sql_value());
            b__878.append_safe(" IN (");
            b__878.append_part(temper_core::ListedTrait::get( & values__875, 0));
            let mut i__879: i32 = 1;
            'loop___11562: loop {
                t___10813 = temper_core::ListedTrait::len( & values__875);
                if ! (Some(i__879) < Some(t___10813)) {
                    break;
                }
                b__878.append_safe(", ");
                b__878.append_part(temper_core::ListedTrait::get( & values__875, i__879));
                i__879 = i__879.wrapping_add(1);
            }
            b__878.append_safe(")");
            t___10818 = b__878.accumulated();
            return__403 = self.r#where(t___10818.clone());
        }
        return return__403.clone();
    }
    pub fn where_in_subquery(& self, field__881: SafeIdentifier, sub__882: Query) -> Query {
        let b__884: SqlBuilder = SqlBuilder::new();
        b__884.append_safe(field__881.sql_value());
        b__884.append_safe(" IN (");
        b__884.append_fragment(sub__882.to_sql());
        b__884.append_safe(")");
        let mut t___10800: SqlFragment = b__884.accumulated();
        return self.r#where(t___10800.clone());
    }
    pub fn where_not(& self, condition__886: SqlFragment) -> Query {
        let b__888: SqlBuilder = SqlBuilder::new();
        b__888.append_safe("NOT (");
        b__888.append_fragment(condition__886.clone());
        b__888.append_safe(")");
        let mut t___10791: SqlFragment = b__888.accumulated();
        return self.r#where(t___10791.clone());
    }
    pub fn where_between(& self, field__890: SafeIdentifier, low__891: SqlPart, high__892: SqlPart) -> Query {
        let b__894: SqlBuilder = SqlBuilder::new();
        b__894.append_safe(field__890.sql_value());
        b__894.append_safe(" BETWEEN ");
        b__894.append_part(low__891.clone());
        b__894.append_safe(" AND ");
        b__894.append_part(high__892.clone());
        let mut t___10785: SqlFragment = b__894.accumulated();
        return self.r#where(t___10785.clone());
    }
    pub fn where_like(& self, field__896: SafeIdentifier, pattern__897: impl temper_core::ToArcString) -> Query {
        let pattern__897 = pattern__897.to_arc_string();
        let b__899: SqlBuilder = SqlBuilder::new();
        b__899.append_safe(field__896.sql_value());
        b__899.append_safe(" LIKE ");
        b__899.append_string(pattern__897.clone());
        let mut t___10776: SqlFragment = b__899.accumulated();
        return self.r#where(t___10776.clone());
    }
    pub fn where_i_like(& self, field__901: SafeIdentifier, pattern__902: impl temper_core::ToArcString) -> Query {
        let pattern__902 = pattern__902.to_arc_string();
        let b__904: SqlBuilder = SqlBuilder::new();
        b__904.append_safe(field__901.sql_value());
        b__904.append_safe(" ILIKE ");
        b__904.append_string(pattern__902.clone());
        let mut t___10769: SqlFragment = b__904.accumulated();
        return self.r#where(t___10769.clone());
    }
    pub fn select(& self, fields__906: impl temper_core::ToList<SafeIdentifier>) -> Query {
        let fields__906 = fields__906.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), fields__906.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn select_expr(& self, exprs__909: impl temper_core::ToList<SqlFragment>) -> Query {
        let exprs__909 = exprs__909.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, exprs__909.clone(), self.0.lock_mode.clone());
    }
    pub fn order_by(& self, field__912: SafeIdentifier, ascending__913: bool) -> Query {
        let nb__915: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__915, OrderClause::new(field__912.clone(), ascending__913, None), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__915), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn order_by_nulls(& self, field__917: SafeIdentifier, ascending__918: bool, nulls__919: NullsPosition) -> Query {
        let nb__921: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__921, OrderClause::new(field__917.clone(), ascending__918, Some(nulls__919.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__921), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn limit(& self, n__923: i32) -> temper_core::Result<Query> {
        if Some(n__923) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), Some(n__923), self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone()));
    }
    pub fn offset(& self, n__926: i32) -> temper_core::Result<Query> {
        if Some(n__926) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, Some(n__926), self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone()));
    }
    pub fn join(& self, joinType__929: JoinType, table__930: SafeIdentifier, onCondition__931: SqlFragment) -> Query {
        let nb__933: temper_core::ListBuilder<JoinClause> = temper_core::ListedTrait::to_list_builder( & self.0.join_clauses);
        temper_core::listed::add( & nb__933, JoinClause::new(joinType__929.clone(), table__930.clone(), Some(onCondition__931.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, temper_core::ListedTrait::to_list( & nb__933), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn inner_join(& self, table__935: SafeIdentifier, onCondition__936: SqlFragment) -> Query {
        let mut t___10731: InnerJoin = InnerJoin::new();
        return self.join(JoinType::new(t___10731.clone()), table__935.clone(), onCondition__936.clone());
    }
    pub fn left_join(& self, table__939: SafeIdentifier, onCondition__940: SqlFragment) -> Query {
        let mut t___10729: LeftJoin = LeftJoin::new();
        return self.join(JoinType::new(t___10729.clone()), table__939.clone(), onCondition__940.clone());
    }
    pub fn right_join(& self, table__943: SafeIdentifier, onCondition__944: SqlFragment) -> Query {
        let mut t___10727: RightJoin = RightJoin::new();
        return self.join(JoinType::new(t___10727.clone()), table__943.clone(), onCondition__944.clone());
    }
    pub fn full_join(& self, table__947: SafeIdentifier, onCondition__948: SqlFragment) -> Query {
        let mut t___10725: FullJoin = FullJoin::new();
        return self.join(JoinType::new(t___10725.clone()), table__947.clone(), onCondition__948.clone());
    }
    pub fn cross_join(& self, table__951: SafeIdentifier) -> Query {
        let nb__953: temper_core::ListBuilder<JoinClause> = temper_core::ListedTrait::to_list_builder( & self.0.join_clauses);
        temper_core::listed::add( & nb__953, JoinClause::new(JoinType::new(CrossJoin::new()), table__951.clone(), None), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, temper_core::ListedTrait::to_list( & nb__953), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn group_by(& self, field__955: SafeIdentifier) -> Query {
        let nb__957: temper_core::ListBuilder<SafeIdentifier> = temper_core::ListedTrait::to_list_builder( & self.0.group_by_fields);
        temper_core::listed::add( & nb__957, field__955.clone(), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), temper_core::ListedTrait::to_list( & nb__957), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn having(& self, condition__959: SqlFragment) -> Query {
        let nb__961: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.having_conditions);
        temper_core::listed::add( & nb__961, WhereClause::new(AndCondition::new(condition__959.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), temper_core::ListedTrait::to_list( & nb__961), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn or_having(& self, condition__963: SqlFragment) -> Query {
        let nb__965: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.having_conditions);
        temper_core::listed::add( & nb__965, WhereClause::new(OrCondition::new(condition__963.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), temper_core::ListedTrait::to_list( & nb__965), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn distinct(& self) -> Query {
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), true, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn lock(& self, mode__969: LockMode) -> Query {
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), Some(mode__969.clone()));
    }
    pub fn to_sql(& self) -> SqlFragment {
        let mut t___10616: i32;
        let mut t___10635: i32;
        let mut t___10654: i32;
        let b__973: SqlBuilder = SqlBuilder::new();
        if self.0.is_distinct {
            b__973.append_safe("SELECT DISTINCT ");
        } else {
            b__973.append_safe("SELECT ");
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.select_exprs) {
            b__973.append_fragment(temper_core::ListedTrait::get( & self.0.select_exprs, 0));
            let mut i__974: i32 = 1;
            'loop___11599: loop {
                t___10616 = temper_core::ListedTrait::len( & self.0.select_exprs);
                if ! (Some(i__974) < Some(t___10616)) {
                    break;
                }
                b__973.append_safe(", ");
                b__973.append_fragment(temper_core::ListedTrait::get( & self.0.select_exprs, i__974));
                i__974 = i__974.wrapping_add(1);
            }
        } else {
            if temper_core::ListedTrait::is_empty( & self.0.selected_fields) {
                b__973.append_safe("*");
            } else {
                #[derive(Clone)]
                struct ClosureGroup___4 {}
                impl ClosureGroup___4 {
                    fn fn__10609(& self, f__975: SafeIdentifier) -> std::sync::Arc<String> {
                        return f__975.sql_value();
                    }
                }
                let closure_group = ClosureGroup___4 {};
                let fn__10609 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | f__975: SafeIdentifier | closure_group.fn__10609(f__975))
                };
                b__973.append_safe(temper_core::listed::join( & self.0.selected_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__10609.clone())));
            }
        }
        b__973.append_safe(" FROM ");
        b__973.append_safe(self.0.table_name.sql_value());
        #[derive(Clone)]
        struct ClosureGroup___5 {
            b__973: SqlBuilder
        }
        impl ClosureGroup___5 {
            fn fn__10608(& self, jc__976: JoinClause) {
                self.b__973.append_safe(" ");
                let mut t___10596: std::sync::Arc<String> = jc__976.join_type().keyword();
                self.b__973.append_safe(t___10596.clone());
                self.b__973.append_safe(" ");
                let mut t___10600: std::sync::Arc<String> = jc__976.table().sql_value();
                self.b__973.append_safe(t___10600.clone());
                let oc__977: Option<SqlFragment> = jc__976.on_condition();
                if ! oc__977.is_none() {
                    let oc___2069: SqlFragment = oc__977.clone().unwrap();
                    self.b__973.append_safe(" ON ");
                    self.b__973.append_fragment(oc___2069.clone());
                }
            }
        }
        let closure_group = ClosureGroup___5 {
            b__973: b__973.clone()
        };
        let fn__10608 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | jc__976: JoinClause | closure_group.fn__10608(jc__976))
        };
        temper_core::listed::list_for_each( & self.0.join_clauses, & ( * fn__10608.clone()));
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__973.append_safe(" WHERE ");
            b__973.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
            let mut i__978: i32 = 1;
            'loop___11601: loop {
                t___10635 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__978) < Some(t___10635)) {
                    break;
                }
                b__973.append_safe(" ");
                b__973.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__978).keyword());
                b__973.append_safe(" ");
                b__973.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__978).condition());
                i__978 = i__978.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.group_by_fields) {
            b__973.append_safe(" GROUP BY ");
            #[derive(Clone)]
            struct ClosureGroup___6 {}
            impl ClosureGroup___6 {
                fn fn__10607(& self, f__979: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__979.sql_value();
                }
            }
            let closure_group = ClosureGroup___6 {};
            let fn__10607 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__979: SafeIdentifier | closure_group.fn__10607(f__979))
            };
            b__973.append_safe(temper_core::listed::join( & self.0.group_by_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__10607.clone())));
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.having_conditions) {
            b__973.append_safe(" HAVING ");
            b__973.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, 0).condition());
            let mut i__980: i32 = 1;
            'loop___11602: loop {
                t___10654 = temper_core::ListedTrait::len( & self.0.having_conditions);
                if ! (Some(i__980) < Some(t___10654)) {
                    break;
                }
                b__973.append_safe(" ");
                b__973.append_safe(temper_core::ListedTrait::get( & self.0.having_conditions, i__980).keyword());
                b__973.append_safe(" ");
                b__973.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, i__980).condition());
                i__980 = i__980.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.order_clauses) {
            b__973.append_safe(" ORDER BY ");
            let mut first__981: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___7 {
                first__981: std::sync::Arc<std::sync::RwLock<bool>>, b__973: SqlBuilder
            }
            impl ClosureGroup___7 {
                fn fn__10606(& self, orc__982: OrderClause) {
                    let mut t___10591: std::sync::Arc<String>;
                    let mut t___5801: std::sync::Arc<String>;
                    if ! temper_core::read_locked( & self.first__981) {
                        self.b__973.append_safe(", ");
                    }
                    {
                        * self.first__981.write().unwrap() = false;
                    }
                    let mut t___10586: std::sync::Arc<String> = orc__982.field().sql_value();
                    self.b__973.append_safe(t___10586.clone());
                    if orc__982.ascending() {
                        t___5801 = std::sync::Arc::new(" ASC".to_string());
                    } else {
                        t___5801 = std::sync::Arc::new(" DESC".to_string());
                    }
                    self.b__973.append_safe(t___5801.clone());
                    let np__983: Option<NullsPosition> = orc__982.nulls_pos();
                    if ! np__983.is_none() {
                        t___10591 = np__983.clone().unwrap().keyword();
                        self.b__973.append_safe(t___10591.clone());
                    }
                }
            }
            let closure_group = ClosureGroup___7 {
                first__981: first__981.clone(), b__973: b__973.clone()
            };
            let fn__10606 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | orc__982: OrderClause | closure_group.fn__10606(orc__982))
            };
            temper_core::listed::list_for_each( & self.0.order_clauses, & ( * fn__10606.clone()));
        }
        let lv__984: Option<i32> = self.0.limit_val;
        if ! lv__984.is_none() {
            let lv___2071: i32 = lv__984.unwrap();
            b__973.append_safe(" LIMIT ");
            b__973.append_int32(lv___2071);
        }
        let ov__985: Option<i32> = self.0.offset_val;
        if ! ov__985.is_none() {
            let ov___2072: i32 = ov__985.unwrap();
            b__973.append_safe(" OFFSET ");
            b__973.append_int32(ov___2072);
        }
        let lm__986: Option<LockMode> = self.0.lock_mode.clone();
        if ! lm__986.is_none() {
            b__973.append_safe(lm__986.clone().unwrap().keyword());
        }
        return b__973.accumulated();
    }
    pub fn count_sql(& self) -> SqlFragment {
        let mut t___10555: i32;
        let mut t___10574: i32;
        let b__989: SqlBuilder = SqlBuilder::new();
        b__989.append_safe("SELECT COUNT(*) FROM ");
        b__989.append_safe(self.0.table_name.sql_value());
        #[derive(Clone)]
        struct ClosureGroup___8 {
            b__989: SqlBuilder
        }
        impl ClosureGroup___8 {
            fn fn__10543(& self, jc__990: JoinClause) {
                self.b__989.append_safe(" ");
                let mut t___10533: std::sync::Arc<String> = jc__990.join_type().keyword();
                self.b__989.append_safe(t___10533.clone());
                self.b__989.append_safe(" ");
                let mut t___10537: std::sync::Arc<String> = jc__990.table().sql_value();
                self.b__989.append_safe(t___10537.clone());
                let oc2__991: Option<SqlFragment> = jc__990.on_condition();
                if ! oc2__991.is_none() {
                    let oc2___2074: SqlFragment = oc2__991.clone().unwrap();
                    self.b__989.append_safe(" ON ");
                    self.b__989.append_fragment(oc2___2074.clone());
                }
            }
        }
        let closure_group = ClosureGroup___8 {
            b__989: b__989.clone()
        };
        let fn__10543 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | jc__990: JoinClause | closure_group.fn__10543(jc__990))
        };
        temper_core::listed::list_for_each( & self.0.join_clauses, & ( * fn__10543.clone()));
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__989.append_safe(" WHERE ");
            b__989.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
            let mut i__992: i32 = 1;
            'loop___11608: loop {
                t___10555 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__992) < Some(t___10555)) {
                    break;
                }
                b__989.append_safe(" ");
                b__989.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__992).keyword());
                b__989.append_safe(" ");
                b__989.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__992).condition());
                i__992 = i__992.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.group_by_fields) {
            b__989.append_safe(" GROUP BY ");
            #[derive(Clone)]
            struct ClosureGroup___9 {}
            impl ClosureGroup___9 {
                fn fn__10542(& self, f__993: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__993.sql_value();
                }
            }
            let closure_group = ClosureGroup___9 {};
            let fn__10542 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__993: SafeIdentifier | closure_group.fn__10542(f__993))
            };
            b__989.append_safe(temper_core::listed::join( & self.0.group_by_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__10542.clone())));
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.having_conditions) {
            b__989.append_safe(" HAVING ");
            b__989.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, 0).condition());
            let mut i__994: i32 = 1;
            'loop___11609: loop {
                t___10574 = temper_core::ListedTrait::len( & self.0.having_conditions);
                if ! (Some(i__994) < Some(t___10574)) {
                    break;
                }
                b__989.append_safe(" ");
                b__989.append_safe(temper_core::ListedTrait::get( & self.0.having_conditions, i__994).keyword());
                b__989.append_safe(" ");
                b__989.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, i__994).condition());
                i__994 = i__994.wrapping_add(1);
            }
        }
        return b__989.accumulated();
    }
    pub fn safe_to_sql(& self, defaultLimit__996: i32) -> temper_core::Result<SqlFragment> {
        let return__428: SqlFragment;
        let mut t___5751: Query;
        if Some(defaultLimit__996) < Some(0) {
            return Err(temper_core::Error::new());
        }
        if ! self.0.limit_val.is_none() {
            return__428 = self.to_sql();
        } else {
            t___5751 = self.limit(defaultLimit__996) ? ;
            return__428 = t___5751.to_sql();
        }
        return Ok(return__428.clone());
    }
    pub fn new(tableName__999: SafeIdentifier, conditions__1000: impl temper_core::ToList<WhereClause>, selectedFields__1001: impl temper_core::ToList<SafeIdentifier>, orderClauses__1002: impl temper_core::ToList<OrderClause>, limitVal__1003: Option<i32>, offsetVal__1004: Option<i32>, joinClauses__1005: impl temper_core::ToList<JoinClause>, groupByFields__1006: impl temper_core::ToList<SafeIdentifier>, havingConditions__1007: impl temper_core::ToList<WhereClause>, isDistinct__1008: bool, selectExprs__1009: impl temper_core::ToList<SqlFragment>, lockMode__1010: Option<LockMode>) -> Query {
        let conditions__1000 = conditions__1000.to_list();
        let selectedFields__1001 = selectedFields__1001.to_list();
        let orderClauses__1002 = orderClauses__1002.to_list();
        let joinClauses__1005 = joinClauses__1005.to_list();
        let groupByFields__1006 = groupByFields__1006.to_list();
        let havingConditions__1007 = havingConditions__1007.to_list();
        let selectExprs__1009 = selectExprs__1009.to_list();
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
        table_name = tableName__999.clone();
        conditions = conditions__1000.clone();
        selected_fields = selectedFields__1001.clone();
        order_clauses = orderClauses__1002.clone();
        limit_val = limitVal__1003;
        offset_val = offsetVal__1004;
        join_clauses = joinClauses__1005.clone();
        group_by_fields = groupByFields__1006.clone();
        having_conditions = havingConditions__1007.clone();
        is_distinct = isDistinct__1008;
        select_exprs = selectExprs__1009.clone();
        lock_mode = lockMode__1010.clone();
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
    pub fn new(field__1060: SafeIdentifier, value__1061: SqlPart) -> SetClause {
        let field;
        let value;
        field = field__1060.clone();
        value = value__1061.clone();
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
    pub fn set(& self, field__1067: SafeIdentifier, value__1068: SqlPart) -> UpdateQuery {
        let nb__1070: temper_core::ListBuilder<SetClause> = temper_core::ListedTrait::to_list_builder( & self.0.set_clauses);
        temper_core::listed::add( & nb__1070, SetClause::new(field__1067.clone(), value__1068.clone()), None);
        return UpdateQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1070), self.0.conditions.clone(), self.0.limit_val);
    }
    pub fn r#where(& self, condition__1072: SqlFragment) -> UpdateQuery {
        let nb__1074: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1074, WhereClause::new(AndCondition::new(condition__1072.clone())), None);
        return UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), temper_core::ListedTrait::to_list( & nb__1074), self.0.limit_val);
    }
    pub fn or_where(& self, condition__1076: SqlFragment) -> UpdateQuery {
        let nb__1078: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1078, WhereClause::new(OrCondition::new(condition__1076.clone())), None);
        return UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), temper_core::ListedTrait::to_list( & nb__1078), self.0.limit_val);
    }
    pub fn limit(& self, n__1080: i32) -> temper_core::Result<UpdateQuery> {
        if Some(n__1080) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), self.0.conditions.clone(), Some(n__1080)));
    }
    pub fn to_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___10390: i32;
        let mut t___10404: i32;
        if temper_core::ListedTrait::is_empty( & self.0.conditions) {
            return Err(temper_core::Error::new());
        }
        if temper_core::ListedTrait::is_empty( & self.0.set_clauses) {
            return Err(temper_core::Error::new());
        }
        let b__1084: SqlBuilder = SqlBuilder::new();
        b__1084.append_safe("UPDATE ");
        b__1084.append_safe(self.0.table_name.sql_value());
        b__1084.append_safe(" SET ");
        b__1084.append_safe(temper_core::ListedTrait::get( & self.0.set_clauses, 0).field().sql_value());
        b__1084.append_safe(" = ");
        b__1084.append_part(temper_core::ListedTrait::get( & self.0.set_clauses, 0).value());
        let mut i__1085: i32 = 1;
        'loop___11619: loop {
            t___10390 = temper_core::ListedTrait::len( & self.0.set_clauses);
            if ! (Some(i__1085) < Some(t___10390)) {
                break;
            }
            b__1084.append_safe(", ");
            b__1084.append_safe(temper_core::ListedTrait::get( & self.0.set_clauses, i__1085).field().sql_value());
            b__1084.append_safe(" = ");
            b__1084.append_part(temper_core::ListedTrait::get( & self.0.set_clauses, i__1085).value());
            i__1085 = i__1085.wrapping_add(1);
        }
        b__1084.append_safe(" WHERE ");
        b__1084.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
        let mut i__1086: i32 = 1;
        'loop___11620: loop {
            t___10404 = temper_core::ListedTrait::len( & self.0.conditions);
            if ! (Some(i__1086) < Some(t___10404)) {
                break;
            }
            b__1084.append_safe(" ");
            b__1084.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__1086).keyword());
            b__1084.append_safe(" ");
            b__1084.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__1086).condition());
            i__1086 = i__1086.wrapping_add(1);
        }
        let lv__1087: Option<i32> = self.0.limit_val;
        if ! lv__1087.is_none() {
            let lv___2075: i32 = lv__1087.unwrap();
            b__1084.append_safe(" LIMIT ");
            b__1084.append_int32(lv___2075);
        }
        return Ok(b__1084.accumulated());
    }
    pub fn new(tableName__1089: SafeIdentifier, setClauses__1090: impl temper_core::ToList<SetClause>, conditions__1091: impl temper_core::ToList<WhereClause>, limitVal__1092: Option<i32>) -> UpdateQuery {
        let setClauses__1090 = setClauses__1090.to_list();
        let conditions__1091 = conditions__1091.to_list();
        let table_name;
        let set_clauses;
        let conditions;
        let limit_val;
        table_name = tableName__1089.clone();
        set_clauses = setClauses__1090.clone();
        conditions = conditions__1091.clone();
        limit_val = limitVal__1092;
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
    pub fn r#where(& self, condition__1097: SqlFragment) -> DeleteQuery {
        let nb__1099: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1099, WhereClause::new(AndCondition::new(condition__1097.clone())), None);
        return DeleteQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1099), self.0.limit_val);
    }
    pub fn or_where(& self, condition__1101: SqlFragment) -> DeleteQuery {
        let nb__1103: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1103, WhereClause::new(OrCondition::new(condition__1101.clone())), None);
        return DeleteQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1103), self.0.limit_val);
    }
    pub fn limit(& self, n__1105: i32) -> temper_core::Result<DeleteQuery> {
        if Some(n__1105) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(DeleteQuery::new(self.0.table_name.clone(), self.0.conditions.clone(), Some(n__1105)));
    }
    pub fn to_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___10350: i32;
        if temper_core::ListedTrait::is_empty( & self.0.conditions) {
            return Err(temper_core::Error::new());
        }
        let b__1109: SqlBuilder = SqlBuilder::new();
        b__1109.append_safe("DELETE FROM ");
        b__1109.append_safe(self.0.table_name.sql_value());
        b__1109.append_safe(" WHERE ");
        b__1109.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
        let mut i__1110: i32 = 1;
        'loop___11626: loop {
            t___10350 = temper_core::ListedTrait::len( & self.0.conditions);
            if ! (Some(i__1110) < Some(t___10350)) {
                break;
            }
            b__1109.append_safe(" ");
            b__1109.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__1110).keyword());
            b__1109.append_safe(" ");
            b__1109.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__1110).condition());
            i__1110 = i__1110.wrapping_add(1);
        }
        let lv__1111: Option<i32> = self.0.limit_val;
        if ! lv__1111.is_none() {
            let lv___2076: i32 = lv__1111.unwrap();
            b__1109.append_safe(" LIMIT ");
            b__1109.append_int32(lv___2076);
        }
        return Ok(b__1109.accumulated());
    }
    pub fn new(tableName__1113: SafeIdentifier, conditions__1114: impl temper_core::ToList<WhereClause>, limitVal__1115: Option<i32>) -> DeleteQuery {
        let conditions__1114 = conditions__1114.to_list();
        let table_name;
        let conditions;
        let limit_val;
        table_name = tableName__1113.clone();
        conditions = conditions__1114.clone();
        limit_val = limitVal__1115;
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
    pub fn new(_value__1350: impl temper_core::ToArcString) -> ValidatedIdentifier {
        let _value__1350 = _value__1350.to_arc_string();
        let value;
        value = _value__1350.clone();
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
    name: SafeIdentifier, field_type: FieldType, nullable: bool
}
#[derive(Clone)]
pub struct FieldDef(std::sync::Arc<FieldDefStruct>);
#[derive(Clone)]
pub struct FieldDefBuilder {
    pub name: SafeIdentifier, pub field_type: FieldType, pub nullable: bool
}
impl FieldDefBuilder {
    pub fn build(self) -> FieldDef {
        FieldDef::new(self.name, self.field_type, self.nullable)
    }
}
impl FieldDef {
    pub fn new(name__1368: SafeIdentifier, fieldType__1369: FieldType, nullable__1370: bool) -> FieldDef {
        let name;
        let field_type;
        let nullable;
        name = name__1368.clone();
        field_type = fieldType__1369.clone();
        nullable = nullable__1370;
        let selfish = FieldDef(std::sync::Arc::new(FieldDefStruct {
                    name, field_type, nullable
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
}
temper_core::impl_any_value_trait!(FieldDef, []);
struct TableDefStruct {
    table_name: SafeIdentifier, fields: temper_core::List<FieldDef>
}
#[derive(Clone)]
pub struct TableDef(std::sync::Arc<TableDefStruct>);
#[derive(Clone)]
pub struct TableDefBuilder {
    pub table_name: SafeIdentifier, pub fields: temper_core::List<FieldDef>
}
impl TableDefBuilder {
    pub fn build(self) -> TableDef {
        TableDef::new(self.table_name, self.fields)
    }
}
impl TableDef {
    pub fn field(& self, name__1374: impl temper_core::ToArcString) -> temper_core::Result<FieldDef> {
        let name__1374 = name__1374.to_arc_string();
        let return__492: FieldDef;
        'fn__1375: {
            let this__6801: temper_core::List<FieldDef> = self.0.fields.clone();
            let n__6802: i32 = temper_core::ListedTrait::len( & this__6801);
            let mut i__6803: i32 = 0;
            'loop___11630: while Some(i__6803) < Some(n__6802) {
                let el__6804: FieldDef = temper_core::ListedTrait::get( & this__6801, i__6803);
                i__6803 = i__6803.wrapping_add(1);
                let f__1376: FieldDef = el__6804.clone();
                if Some(f__1376.name().sql_value().as_str()) == Some(name__1374.as_str()) {
                    return__492 = f__1376.clone();
                    break 'fn__1375;
                }
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__492.clone());
    }
    pub fn new(tableName__1378: SafeIdentifier, fields__1379: impl temper_core::ToList<FieldDef>) -> TableDef {
        let fields__1379 = fields__1379.to_list();
        let table_name;
        let fields;
        table_name = tableName__1378.clone();
        fields = fields__1379.clone();
        let selfish = TableDef(std::sync::Arc::new(TableDefStruct {
                    table_name, fields
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn fields(& self) -> temper_core::List<FieldDef> {
        return self.0.fields.clone();
    }
}
temper_core::impl_any_value_trait!(TableDef, []);
struct SqlBuilderStruct {
    buffer: temper_core::ListBuilder<SqlPart>
}
#[derive(Clone)]
pub struct SqlBuilder(std::sync::Arc<SqlBuilderStruct>);
impl SqlBuilder {
    pub fn append_safe(& self, sqlSource__1401: impl temper_core::ToArcString) {
        let sqlSource__1401 = sqlSource__1401.to_arc_string();
        let mut t___11421: SqlSource = SqlSource::new(sqlSource__1401.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___11421.clone()), None);
    }
    pub fn append_fragment(& self, fragment__1404: SqlFragment) {
        let mut t___11419: temper_core::List<SqlPart> = fragment__1404.parts();
        temper_core::listed::add_all( & self.0.buffer, temper_core::ToListed::to_listed(t___11419.clone()), None);
    }
    pub fn append_part(& self, part__1407: SqlPart) {
        temper_core::listed::add( & self.0.buffer, part__1407.clone(), None);
    }
    pub fn append_part_list(& self, values__1410: impl temper_core::ToList<SqlPart>) {
        let values__1410 = values__1410.to_list();
        #[derive(Clone)]
        struct ClosureGroup___10 {
            this__262: SqlBuilder
        }
        impl ClosureGroup___10 {
            fn fn__11415(& self, x__1412: SqlPart) {
                self.this__262.append_part(x__1412.clone());
            }
        }
        let closure_group = ClosureGroup___10 {
            this__262: self.clone()
        };
        let fn__11415 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1412: SqlPart | closure_group.fn__11415(x__1412))
        };
        self.append_list(temper_core::ToListed::to_listed(values__1410.clone()), fn__11415.clone());
    }
    pub fn append_boolean(& self, value__1414: bool) {
        let mut t___11412: SqlBoolean = SqlBoolean::new(value__1414);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___11412.clone()), None);
    }
    pub fn append_boolean_list(& self, values__1417: impl temper_core::ToListed<bool>) {
        let values__1417 = values__1417.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___11 {
            this__264: SqlBuilder
        }
        impl ClosureGroup___11 {
            fn fn__11409(& self, x__1419: bool) {
                self.this__264.append_boolean(x__1419);
            }
        }
        let closure_group = ClosureGroup___11 {
            this__264: self.clone()
        };
        let fn__11409 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1419: bool | closure_group.fn__11409(x__1419))
        };
        self.append_list(values__1417.clone(), fn__11409.clone());
    }
    pub fn append_date(& self, value__1421: temper_std::temporal::Date) {
        let mut t___11406: SqlDate = SqlDate::new(value__1421.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___11406.clone()), None);
    }
    pub fn append_date_list(& self, values__1424: impl temper_core::ToListed<temper_std::temporal::Date>) {
        let values__1424 = values__1424.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___12 {
            this__266: SqlBuilder
        }
        impl ClosureGroup___12 {
            fn fn__11403(& self, x__1426: temper_std::temporal::Date) {
                self.this__266.append_date(x__1426.clone());
            }
        }
        let closure_group = ClosureGroup___12 {
            this__266: self.clone()
        };
        let fn__11403 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1426: temper_std::temporal::Date | closure_group.fn__11403(x__1426))
        };
        self.append_list(values__1424.clone(), fn__11403.clone());
    }
    pub fn append_float64(& self, value__1428: f64) {
        let mut t___11400: SqlFloat64 = SqlFloat64::new(value__1428);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___11400.clone()), None);
    }
    pub fn append_float64_list(& self, values__1431: impl temper_core::ToListed<f64>) {
        let values__1431 = values__1431.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___13 {
            this__268: SqlBuilder
        }
        impl ClosureGroup___13 {
            fn fn__11397(& self, x__1433: f64) {
                self.this__268.append_float64(x__1433);
            }
        }
        let closure_group = ClosureGroup___13 {
            this__268: self.clone()
        };
        let fn__11397 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1433: f64 | closure_group.fn__11397(x__1433))
        };
        self.append_list(values__1431.clone(), fn__11397.clone());
    }
    pub fn append_int32(& self, value__1435: i32) {
        let mut t___11394: SqlInt32 = SqlInt32::new(value__1435);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___11394.clone()), None);
    }
    pub fn append_int32_list(& self, values__1438: impl temper_core::ToListed<i32>) {
        let values__1438 = values__1438.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___14 {
            this__270: SqlBuilder
        }
        impl ClosureGroup___14 {
            fn fn__11391(& self, x__1440: i32) {
                self.this__270.append_int32(x__1440);
            }
        }
        let closure_group = ClosureGroup___14 {
            this__270: self.clone()
        };
        let fn__11391 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1440: i32 | closure_group.fn__11391(x__1440))
        };
        self.append_list(values__1438.clone(), fn__11391.clone());
    }
    pub fn append_int64(& self, value__1442: i64) {
        let mut t___11388: SqlInt64 = SqlInt64::new(value__1442);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___11388.clone()), None);
    }
    pub fn append_int64_list(& self, values__1445: impl temper_core::ToListed<i64>) {
        let values__1445 = values__1445.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___15 {
            this__272: SqlBuilder
        }
        impl ClosureGroup___15 {
            fn fn__11385(& self, x__1447: i64) {
                self.this__272.append_int64(x__1447);
            }
        }
        let closure_group = ClosureGroup___15 {
            this__272: self.clone()
        };
        let fn__11385 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1447: i64 | closure_group.fn__11385(x__1447))
        };
        self.append_list(values__1445.clone(), fn__11385.clone());
    }
    pub fn append_string(& self, value__1449: impl temper_core::ToArcString) {
        let value__1449 = value__1449.to_arc_string();
        let mut t___11382: SqlString = SqlString::new(value__1449.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___11382.clone()), None);
    }
    pub fn append_string_list(& self, values__1452: impl temper_core::ToListed<std::sync::Arc<String>>) {
        let values__1452 = values__1452.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___16 {
            this__274: SqlBuilder
        }
        impl ClosureGroup___16 {
            fn fn__11379(& self, x__1454: impl temper_core::ToArcString) {
                let x__1454 = x__1454.to_arc_string();
                self.this__274.append_string(x__1454.clone());
            }
        }
        let closure_group = ClosureGroup___16 {
            this__274: self.clone()
        };
        let fn__11379 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1454: std::sync::Arc<String> | closure_group.fn__11379(x__1454))
        };
        self.append_list(values__1452.clone(), fn__11379.clone());
    }
    fn append_list<T>(& self, values__1456: impl temper_core::ToListed<T>, appendValue__1457: std::sync::Arc<dyn Fn (T) + std::marker::Send + std::marker::Sync>) where T: Clone + std::marker::Send + std::marker::Sync + 'static {
        let values__1456 = values__1456.to_listed();
        let mut t___11374: i32;
        let mut t___11376: T;
        let mut i__1459: i32 = 0;
        'loop___11631: loop {
            t___11374 = temper_core::ListedTrait::len( & ( * values__1456));
            if ! (Some(i__1459) < Some(t___11374)) {
                break;
            }
            if Some(i__1459) > Some(0) {
                self.append_safe(", ");
            }
            t___11376 = temper_core::ListedTrait::get( & ( * values__1456), i__1459);
            appendValue__1457(t___11376.clone());
            i__1459 = i__1459.wrapping_add(1);
        }
    }
    pub fn accumulated(& self) -> SqlFragment {
        return SqlFragment::new(temper_core::ListedTrait::to_list( & self.0.buffer));
    }
    pub fn new() -> SqlBuilder {
        let buffer;
        let mut t___11371: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        buffer = t___11371.clone();
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
        let mut t___11445: i32;
        let builder__1471: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        let mut i__1472: i32 = 0;
        'loop___11632: loop {
            t___11445 = temper_core::ListedTrait::len( & self.0.parts);
            if ! (Some(i__1472) < Some(t___11445)) {
                break;
            }
            temper_core::ListedTrait::get( & self.0.parts, i__1472).format_to(builder__1471.clone());
            i__1472 = i__1472.wrapping_add(1);
        }
        return temper_core::string::builder::to_string( & builder__1471);
    }
    pub fn new(parts__1474: impl temper_core::ToList<SqlPart>) -> SqlFragment {
        let parts__1474 = parts__1474.to_list();
        let parts;
        parts = parts__1474.clone();
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
    SqlSource(SqlSource), SqlBoolean(SqlBoolean), SqlString(SqlString), SqlInt32(SqlInt32), SqlInt64(SqlInt64), SqlFloat64(SqlFloat64), SqlDate(SqlDate)
}
pub trait SqlPartTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> SqlPartEnum;
    fn clone_boxed(& self) -> SqlPart;
    fn format_to(& self, builder__1476: std::sync::Arc<std::sync::RwLock<String>>);
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
    pub fn format_to(& self, builder__1480: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1480, self.0.source.clone());
    }
    pub fn new(source__1483: impl temper_core::ToArcString) -> SqlSource {
        let source__1483 = source__1483.to_arc_string();
        let source;
        source = source__1483.clone();
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
    fn format_to(& self, builder__1480: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1480)
    }
}
temper_core::impl_any_value_trait!(SqlSource, [SqlPart]);
struct SqlBooleanStruct {
    value: bool
}
#[derive(Clone)]
pub struct SqlBoolean(std::sync::Arc<SqlBooleanStruct>);
impl SqlBoolean {
    pub fn format_to(& self, builder__1486: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___6592: std::sync::Arc<String>;
        if self.0.value {
            t___6592 = std::sync::Arc::new("TRUE".to_string());
        } else {
            t___6592 = std::sync::Arc::new("FALSE".to_string());
        }
        temper_core::string::builder::append( & builder__1486, t___6592.clone());
    }
    pub fn new(value__1489: bool) -> SqlBoolean {
        let value;
        value = value__1489;
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
    fn format_to(& self, builder__1486: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1486)
    }
}
temper_core::impl_any_value_trait!(SqlBoolean, [SqlPart]);
struct SqlDateStruct {
    value: temper_std::temporal::Date
}
#[derive(Clone)]
pub struct SqlDate(std::sync::Arc<SqlDateStruct>);
impl SqlDate {
    pub fn format_to(& self, builder__1492: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1492, "'");
        let mut t___11426: std::sync::Arc<String> = self.0.value.to_string();
        #[derive(Clone)]
        struct ClosureGroup___17 {
            builder__1492: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___17 {
            fn fn__11424(& self, c__1494: i32) {
                if Some(c__1494) == Some(39) {
                    temper_core::string::builder::append( & self.builder__1492, "''");
                } else {
                    'ok___11484: {
                        'orelse___2008: {
                            match temper_core::string::builder::append_code_point( & self.builder__1492, c__1494) {
                                Ok(x) => x,
                                _ => break 'orelse___2008
                            };
                            break 'ok___11484;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___17 {
            builder__1492: builder__1492.clone()
        };
        let fn__11424 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1494: i32 | closure_group.fn__11424(c__1494))
        };
        temper_core::string::for_each( & t___11426, & ( * fn__11424.clone()));
        temper_core::string::builder::append( & builder__1492, "'");
    }
    pub fn new(value__1496: temper_std::temporal::Date) -> SqlDate {
        let value;
        value = value__1496.clone();
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
    fn format_to(& self, builder__1492: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1492)
    }
}
temper_core::impl_any_value_trait!(SqlDate, [SqlPart]);
struct SqlFloat64Struct {
    value: f64
}
#[derive(Clone)]
pub struct SqlFloat64(std::sync::Arc<SqlFloat64Struct>);
impl SqlFloat64 {
    pub fn format_to(& self, builder__1499: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___6581: bool;
        let mut t___6582: bool;
        let s__1501: std::sync::Arc<String> = temper_core::float64::to_string(self.0.value);
        if Some(s__1501.as_str()) == Some("NaN") {
            t___6582 = true;
        } else {
            if Some(s__1501.as_str()) == Some("Infinity") {
                t___6581 = true;
            } else {
                t___6581 = Some(s__1501.as_str()) == Some("-Infinity");
            }
            t___6582 = t___6581;
        }
        if t___6582 {
            temper_core::string::builder::append( & builder__1499, "NULL");
        } else {
            temper_core::string::builder::append( & builder__1499, s__1501.clone());
        }
    }
    pub fn new(value__1503: f64) -> SqlFloat64 {
        let value;
        value = value__1503;
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
    fn format_to(& self, builder__1499: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1499)
    }
}
temper_core::impl_any_value_trait!(SqlFloat64, [SqlPart]);
struct SqlInt32Struct {
    value: i32
}
#[derive(Clone)]
pub struct SqlInt32(std::sync::Arc<SqlInt32Struct>);
impl SqlInt32 {
    pub fn format_to(& self, builder__1506: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___11435: std::sync::Arc<String> = temper_core::int_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__1506, t___11435.clone());
    }
    pub fn new(value__1509: i32) -> SqlInt32 {
        let value;
        value = value__1509;
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
    fn format_to(& self, builder__1506: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1506)
    }
}
temper_core::impl_any_value_trait!(SqlInt32, [SqlPart]);
struct SqlInt64Struct {
    value: i64
}
#[derive(Clone)]
pub struct SqlInt64(std::sync::Arc<SqlInt64Struct>);
impl SqlInt64 {
    pub fn format_to(& self, builder__1512: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___11433: std::sync::Arc<String> = temper_core::int64_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__1512, t___11433.clone());
    }
    pub fn new(value__1515: i64) -> SqlInt64 {
        let value;
        value = value__1515;
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
    fn format_to(& self, builder__1512: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1512)
    }
}
temper_core::impl_any_value_trait!(SqlInt64, [SqlPart]);
struct SqlStringStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlString(std::sync::Arc<SqlStringStruct>);
impl SqlString {
    pub fn format_to(& self, builder__1518: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1518, "'");
        #[derive(Clone)]
        struct ClosureGroup___18 {
            builder__1518: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___18 {
            fn fn__11438(& self, c__1520: i32) {
                if Some(c__1520) == Some(39) {
                    temper_core::string::builder::append( & self.builder__1518, "''");
                } else {
                    'ok___11489: {
                        'orelse___2007: {
                            match temper_core::string::builder::append_code_point( & self.builder__1518, c__1520) {
                                Ok(x) => x,
                                _ => break 'orelse___2007
                            };
                            break 'ok___11489;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___18 {
            builder__1518: builder__1518.clone()
        };
        let fn__11438 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1520: i32 | closure_group.fn__11438(c__1520))
        };
        temper_core::string::for_each( & self.0.value, & ( * fn__11438.clone()));
        temper_core::string::builder::append( & builder__1518, "'");
    }
    pub fn new(value__1522: impl temper_core::ToArcString) -> SqlString {
        let value__1522 = value__1522.to_arc_string();
        let value;
        value = value__1522.clone();
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
    fn format_to(& self, builder__1518: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1518)
    }
}
temper_core::impl_any_value_trait!(SqlString, [SqlPart]);
pub fn changeset(tableDef__686: TableDef, params__687: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>) -> Changeset {
    let mut t___11221: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
    return Changeset::new(ChangesetImpl::new(tableDef__686.clone(), params__687.clone(), t___11221.clone(), [], true));
}
fn isIdentStart__547(c__1351: i32) -> bool {
    let return__472: bool;
    let mut t___6355: bool;
    let mut t___6356: bool;
    if Some(c__1351) >= Some(97) {
        t___6355 = Some(c__1351) <= Some(122);
    } else {
        t___6355 = false;
    }
    if t___6355 {
        return__472 = true;
    } else {
        if Some(c__1351) >= Some(65) {
            t___6356 = Some(c__1351) <= Some(90);
        } else {
            t___6356 = false;
        }
        if t___6356 {
            return__472 = true;
        } else {
            return__472 = Some(c__1351) == Some(95);
        }
    }
    return return__472;
}
fn isIdentPart__548(c__1353: i32) -> bool {
    let return__473: bool;
    if isIdentStart__547(c__1353) {
        return__473 = true;
    } else {
        if Some(c__1353) >= Some(48) {
            return__473 = Some(c__1353) <= Some(57);
        } else {
            return__473 = false;
        }
    }
    return return__473;
}
pub fn safe_identifier(name__1355: impl temper_core::ToArcString) -> temper_core::Result<SafeIdentifier> {
    let name__1355 = name__1355.to_arc_string();
    let mut t___11219: usize;
    if name__1355.is_empty() {
        return Err(temper_core::Error::new());
    }
    let mut idx__1357: usize = 0usize;
    if ! isIdentStart__547(temper_core::string::get( & name__1355, idx__1357)) {
        return Err(temper_core::Error::new());
    }
    let mut t___11216: usize = temper_core::string::next( & name__1355, idx__1357);
    idx__1357 = t___11216;
    'loop___11633: loop {
        if ! temper_core::string::has_index( & name__1355, idx__1357) {
            break;
        }
        if ! isIdentPart__548(temper_core::string::get( & name__1355, idx__1357)) {
            return Err(temper_core::Error::new());
        }
        t___11219 = temper_core::string::next( & name__1355, idx__1357);
        idx__1357 = t___11219;
    }
    return Ok(SafeIdentifier::new(ValidatedIdentifier::new(name__1355.clone())));
}
fn csid__544(name__689: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__689 = name__689.to_arc_string();
    let return__335: SafeIdentifier;
    let mut t___6343: SafeIdentifier;
    'ok___11494: {
        'orelse___2012: {
            t___6343 = match safe_identifier(name__689.clone()) {
                Ok(x) => x,
                _ => break 'orelse___2012
            };
            return__335 = t___6343.clone();
            break 'ok___11494;
        }
        return__335 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__335.clone();
}
fn userTable__545() -> TableDef {
    return TableDef::new(csid__544("users"), [FieldDef::new(csid__544("name"), FieldType::new(StringField::new()), false), FieldDef::new(csid__544("email"), FieldType::new(StringField::new()), false), FieldDef::new(csid__544("age"), FieldType::new(IntField::new()), true), FieldDef::new(csid__544("score"), FieldType::new(FloatField::new()), true), FieldDef::new(csid__544("active"), FieldType::new(BoolField::new()), true)]);
}
pub fn delete_sql(tableDef__776: TableDef, id__777: i32) -> SqlFragment {
    let b__779: SqlBuilder = SqlBuilder::new();
    b__779.append_safe("DELETE FROM ");
    b__779.append_safe(tableDef__776.table_name().sql_value());
    b__779.append_safe(" WHERE id = ");
    b__779.append_int32(id__777);
    return b__779.accumulated();
}
pub fn from(tableName__1011: SafeIdentifier) -> Query {
    return Query::new(tableName__1011.clone(), [], [], [], None, None, [], [], [], false, [], None);
}
pub fn col(table__1013: SafeIdentifier, column__1014: SafeIdentifier) -> SqlFragment {
    let b__1016: SqlBuilder = SqlBuilder::new();
    b__1016.append_safe(table__1013.sql_value());
    b__1016.append_safe(".");
    b__1016.append_safe(column__1014.sql_value());
    return b__1016.accumulated();
}
pub fn count_all() -> SqlFragment {
    let b__1018: SqlBuilder = SqlBuilder::new();
    b__1018.append_safe("COUNT(*)");
    return b__1018.accumulated();
}
pub fn count_col(field__1019: SafeIdentifier) -> SqlFragment {
    let b__1021: SqlBuilder = SqlBuilder::new();
    b__1021.append_safe("COUNT(");
    b__1021.append_safe(field__1019.sql_value());
    b__1021.append_safe(")");
    return b__1021.accumulated();
}
pub fn sum_col(field__1022: SafeIdentifier) -> SqlFragment {
    let b__1024: SqlBuilder = SqlBuilder::new();
    b__1024.append_safe("SUM(");
    b__1024.append_safe(field__1022.sql_value());
    b__1024.append_safe(")");
    return b__1024.accumulated();
}
pub fn avg_col(field__1025: SafeIdentifier) -> SqlFragment {
    let b__1027: SqlBuilder = SqlBuilder::new();
    b__1027.append_safe("AVG(");
    b__1027.append_safe(field__1025.sql_value());
    b__1027.append_safe(")");
    return b__1027.accumulated();
}
pub fn min_col(field__1028: SafeIdentifier) -> SqlFragment {
    let b__1030: SqlBuilder = SqlBuilder::new();
    b__1030.append_safe("MIN(");
    b__1030.append_safe(field__1028.sql_value());
    b__1030.append_safe(")");
    return b__1030.accumulated();
}
pub fn max_col(field__1031: SafeIdentifier) -> SqlFragment {
    let b__1033: SqlBuilder = SqlBuilder::new();
    b__1033.append_safe("MAX(");
    b__1033.append_safe(field__1031.sql_value());
    b__1033.append_safe(")");
    return b__1033.accumulated();
}
pub fn union_sql(a__1034: Query, b__1035: Query) -> SqlFragment {
    let sb__1037: SqlBuilder = SqlBuilder::new();
    sb__1037.append_safe("(");
    sb__1037.append_fragment(a__1034.to_sql());
    sb__1037.append_safe(") UNION (");
    sb__1037.append_fragment(b__1035.to_sql());
    sb__1037.append_safe(")");
    return sb__1037.accumulated();
}
pub fn union_all_sql(a__1038: Query, b__1039: Query) -> SqlFragment {
    let sb__1041: SqlBuilder = SqlBuilder::new();
    sb__1041.append_safe("(");
    sb__1041.append_fragment(a__1038.to_sql());
    sb__1041.append_safe(") UNION ALL (");
    sb__1041.append_fragment(b__1039.to_sql());
    sb__1041.append_safe(")");
    return sb__1041.accumulated();
}
pub fn intersect_sql(a__1042: Query, b__1043: Query) -> SqlFragment {
    let sb__1045: SqlBuilder = SqlBuilder::new();
    sb__1045.append_safe("(");
    sb__1045.append_fragment(a__1042.to_sql());
    sb__1045.append_safe(") INTERSECT (");
    sb__1045.append_fragment(b__1043.to_sql());
    sb__1045.append_safe(")");
    return sb__1045.accumulated();
}
pub fn except_sql(a__1046: Query, b__1047: Query) -> SqlFragment {
    let sb__1049: SqlBuilder = SqlBuilder::new();
    sb__1049.append_safe("(");
    sb__1049.append_fragment(a__1046.to_sql());
    sb__1049.append_safe(") EXCEPT (");
    sb__1049.append_fragment(b__1047.to_sql());
    sb__1049.append_safe(")");
    return sb__1049.accumulated();
}
pub fn subquery(q__1050: Query, alias__1051: SafeIdentifier) -> SqlFragment {
    let b__1053: SqlBuilder = SqlBuilder::new();
    b__1053.append_safe("(");
    b__1053.append_fragment(q__1050.to_sql());
    b__1053.append_safe(") AS ");
    b__1053.append_safe(alias__1051.sql_value());
    return b__1053.accumulated();
}
pub fn exists_sql(q__1054: Query) -> SqlFragment {
    let b__1056: SqlBuilder = SqlBuilder::new();
    b__1056.append_safe("EXISTS (");
    b__1056.append_fragment(q__1054.to_sql());
    b__1056.append_safe(")");
    return b__1056.accumulated();
}
pub fn update(tableName__1116: SafeIdentifier) -> UpdateQuery {
    return UpdateQuery::new(tableName__1116.clone(), [], [], None);
}
pub fn delete_from(tableName__1118: SafeIdentifier) -> DeleteQuery {
    return DeleteQuery::new(tableName__1118.clone(), [], None);
}
fn sid__546(name__1120: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__1120 = name__1120.to_arc_string();
    let return__465: SafeIdentifier;
    let mut t___5526: SafeIdentifier;
    'ok___11505: {
        'orelse___2020: {
            t___5526 = match safe_identifier(name__1120.clone()) {
                Ok(x) => x,
                _ => break 'orelse___2020
            };
            return__465 = t___5526.clone();
            break 'ok___11505;
        }
        return__465 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__465.clone();
}
#[cfg(test)]
mod tests {
    #[test]
    fn castWhitelistsAllowedFields__1645() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___24 = temper_std::testing::Test::new();
        let params__693: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string())), (std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("true".to_string()))]);
        let mut t___11177: TableDef = userTable__545();
        let mut t___11178: SafeIdentifier = csid__544("name");
        let mut t___11179: SafeIdentifier = csid__544("email");
        let cs__694: Changeset = changeset(t___11177.clone(), params__693.clone()).cast(std::sync::Arc::new(vec![t___11178.clone(), t___11179.clone()]));
        let mut t___11182: bool = temper_core::MappedTrait::has( & cs__694.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___19 {}
        impl ClosureGroup___19 {
            fn fn__11172(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___19 {};
        let fn__11172 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11172())
        };
        test___24.assert(t___11182, fn__11172.clone());
        let mut t___11186: bool = temper_core::MappedTrait::has( & cs__694.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___20 {}
        impl ClosureGroup___20 {
            fn fn__11171(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___20 {};
        let fn__11171 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11171())
        };
        test___24.assert(t___11186, fn__11171.clone());
        let mut t___11192: bool = ! temper_core::MappedTrait::has( & cs__694.changes(), std::sync::Arc::new("admin".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___21 {}
        impl ClosureGroup___21 {
            fn fn__11170(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("admin must be dropped (not in whitelist)".to_string());
            }
        }
        let closure_group = ClosureGroup___21 {};
        let fn__11170 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11170())
        };
        test___24.assert(t___11192, fn__11170.clone());
        let mut t___11194: bool = cs__694.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___22 {}
        impl ClosureGroup___22 {
            fn fn__11169(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___22 {};
        let fn__11169 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11169())
        };
        test___24.assert(t___11194, fn__11169.clone());
        test___24.soft_fail_to_hard()
    }
    #[test]
    fn castIsReplacingNotAdditiveSecondCallResetsWhitelist__1646() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___25 = temper_std::testing::Test::new();
        let params__696: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___11155: TableDef = userTable__545();
        let mut t___11156: SafeIdentifier = csid__544("name");
        let cs__697: Changeset = changeset(t___11155.clone(), params__696.clone()).cast(std::sync::Arc::new(vec![t___11156.clone()])).cast(std::sync::Arc::new(vec![csid__544("email")]));
        let mut t___11163: bool = ! temper_core::MappedTrait::has( & cs__697.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___23 {}
        impl ClosureGroup___23 {
            fn fn__11151(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name must be excluded by second cast".to_string());
            }
        }
        let closure_group = ClosureGroup___23 {};
        let fn__11151 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11151())
        };
        test___25.assert(t___11163, fn__11151.clone());
        let mut t___11166: bool = temper_core::MappedTrait::has( & cs__697.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___24 {}
        impl ClosureGroup___24 {
            fn fn__11150(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be present".to_string());
            }
        }
        let closure_group = ClosureGroup___24 {};
        let fn__11150 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11150())
        };
        test___25.assert(t___11166, fn__11150.clone());
        test___25.soft_fail_to_hard()
    }
    #[test]
    fn castIgnoresEmptyStringValues__1647() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___26 = temper_std::testing::Test::new();
        let params__699: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bob@example.com".to_string()))]);
        let mut t___11137: TableDef = userTable__545();
        let mut t___11138: SafeIdentifier = csid__544("name");
        let mut t___11139: SafeIdentifier = csid__544("email");
        let cs__700: Changeset = changeset(t___11137.clone(), params__699.clone()).cast(std::sync::Arc::new(vec![t___11138.clone(), t___11139.clone()]));
        let mut t___11144: bool = ! temper_core::MappedTrait::has( & cs__700.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___25 {}
        impl ClosureGroup___25 {
            fn fn__11133(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty name should not be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___25 {};
        let fn__11133 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11133())
        };
        test___26.assert(t___11144, fn__11133.clone());
        let mut t___11147: bool = temper_core::MappedTrait::has( & cs__700.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___26 {}
        impl ClosureGroup___26 {
            fn fn__11132(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___26 {};
        let fn__11132 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11132())
        };
        test___26.assert(t___11147, fn__11132.clone());
        test___26.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredPassesWhenFieldPresent__1648() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___27 = temper_std::testing::Test::new();
        let params__702: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___11119: TableDef = userTable__545();
        let mut t___11120: SafeIdentifier = csid__544("name");
        let cs__703: Changeset = changeset(t___11119.clone(), params__702.clone()).cast(std::sync::Arc::new(vec![t___11120.clone()])).validate_required(std::sync::Arc::new(vec![csid__544("name")]));
        let mut t___11124: bool = cs__703.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___27 {}
        impl ClosureGroup___27 {
            fn fn__11116(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___27 {};
        let fn__11116 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11116())
        };
        test___27.assert(t___11124, fn__11116.clone());
        let mut t___11130: bool = Some(temper_core::ListedTrait::len( & cs__703.errors())) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___28 {}
        impl ClosureGroup___28 {
            fn fn__11115(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("no errors expected".to_string());
            }
        }
        let closure_group = ClosureGroup___28 {};
        let fn__11115 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11115())
        };
        test___27.assert(t___11130, fn__11115.clone());
        test___27.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredFailsWhenFieldMissing__1649() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___28 = temper_std::testing::Test::new();
        let params__705: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___11095: TableDef = userTable__545();
        let mut t___11096: SafeIdentifier = csid__544("name");
        let cs__706: Changeset = changeset(t___11095.clone(), params__705.clone()).cast(std::sync::Arc::new(vec![t___11096.clone()])).validate_required(std::sync::Arc::new(vec![csid__544("name")]));
        let mut t___11102: bool = ! cs__706.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___29 {}
        impl ClosureGroup___29 {
            fn fn__11093(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___29 {};
        let fn__11093 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11093())
        };
        test___28.assert(t___11102, fn__11093.clone());
        let mut t___11107: bool = Some(temper_core::ListedTrait::len( & cs__706.errors())) == Some(1);
        #[derive(Clone)]
        struct ClosureGroup___30 {}
        impl ClosureGroup___30 {
            fn fn__11092(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have one error".to_string());
            }
        }
        let closure_group = ClosureGroup___30 {};
        let fn__11092 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11092())
        };
        test___28.assert(t___11107, fn__11092.clone());
        let mut t___11113: bool = Some(temper_core::ListedTrait::get( & cs__706.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___31 {}
        impl ClosureGroup___31 {
            fn fn__11091(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error should name the field".to_string());
            }
        }
        let closure_group = ClosureGroup___31 {};
        let fn__11091 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11091())
        };
        test___28.assert(t___11113, fn__11091.clone());
        test___28.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthPassesWithinRange__1650() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___29 = temper_std::testing::Test::new();
        let params__708: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___11083: TableDef = userTable__545();
        let mut t___11084: SafeIdentifier = csid__544("name");
        let cs__709: Changeset = changeset(t___11083.clone(), params__708.clone()).cast(std::sync::Arc::new(vec![t___11084.clone()])).validate_length(csid__544("name"), 2, 50);
        let mut t___11088: bool = cs__709.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___32 {}
        impl ClosureGroup___32 {
            fn fn__11080(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___32 {};
        let fn__11080 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11080())
        };
        test___29.assert(t___11088, fn__11080.clone());
        test___29.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooShort__1651() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___30 = temper_std::testing::Test::new();
        let params__711: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("A".to_string()))]);
        let mut t___11071: TableDef = userTable__545();
        let mut t___11072: SafeIdentifier = csid__544("name");
        let cs__712: Changeset = changeset(t___11071.clone(), params__711.clone()).cast(std::sync::Arc::new(vec![t___11072.clone()])).validate_length(csid__544("name"), 2, 50);
        let mut t___11078: bool = ! cs__712.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___33 {}
        impl ClosureGroup___33 {
            fn fn__11068(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___33 {};
        let fn__11068 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11068())
        };
        test___30.assert(t___11078, fn__11068.clone());
        test___30.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooLong__1652() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___31 = temper_std::testing::Test::new();
        let params__714: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()))]);
        let mut t___11059: TableDef = userTable__545();
        let mut t___11060: SafeIdentifier = csid__544("name");
        let cs__715: Changeset = changeset(t___11059.clone(), params__714.clone()).cast(std::sync::Arc::new(vec![t___11060.clone()])).validate_length(csid__544("name"), 2, 10);
        let mut t___11066: bool = ! cs__715.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___34 {}
        impl ClosureGroup___34 {
            fn fn__11056(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___34 {};
        let fn__11056 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11056())
        };
        test___31.assert(t___11066, fn__11056.clone());
        test___31.soft_fail_to_hard()
    }
    #[test]
    fn validateIntPassesForValidInteger__1653() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let params__717: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("30".to_string()))]);
        let mut t___11048: TableDef = userTable__545();
        let mut t___11049: SafeIdentifier = csid__544("age");
        let cs__718: Changeset = changeset(t___11048.clone(), params__717.clone()).cast(std::sync::Arc::new(vec![t___11049.clone()])).validate_int(csid__544("age"));
        let mut t___11053: bool = cs__718.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___35 {}
        impl ClosureGroup___35 {
            fn fn__11045(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___35 {};
        let fn__11045 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11045())
        };
        test___32.assert(t___11053, fn__11045.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn validateIntFailsForNonInteger__1654() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        let params__720: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___11036: TableDef = userTable__545();
        let mut t___11037: SafeIdentifier = csid__544("age");
        let cs__721: Changeset = changeset(t___11036.clone(), params__720.clone()).cast(std::sync::Arc::new(vec![t___11037.clone()])).validate_int(csid__544("age"));
        let mut t___11043: bool = ! cs__721.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___36 {}
        impl ClosureGroup___36 {
            fn fn__11033(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___36 {};
        let fn__11033 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11033())
        };
        test___33.assert(t___11043, fn__11033.clone());
        test___33.soft_fail_to_hard()
    }
    #[test]
    fn validateFloatPassesForValidFloat__1655() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___34 = temper_std::testing::Test::new();
        let params__723: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("9.5".to_string()))]);
        let mut t___11025: TableDef = userTable__545();
        let mut t___11026: SafeIdentifier = csid__544("score");
        let cs__724: Changeset = changeset(t___11025.clone(), params__723.clone()).cast(std::sync::Arc::new(vec![t___11026.clone()])).validate_float(csid__544("score"));
        let mut t___11030: bool = cs__724.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___37 {}
        impl ClosureGroup___37 {
            fn fn__11022(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___37 {};
        let fn__11022 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11022())
        };
        test___34.assert(t___11030, fn__11022.clone());
        test___34.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_passesForValid64_bitInteger__1656() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___35 = temper_std::testing::Test::new();
        let params__726: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("9999999999".to_string()))]);
        let mut t___11014: TableDef = userTable__545();
        let mut t___11015: SafeIdentifier = csid__544("age");
        let cs__727: Changeset = changeset(t___11014.clone(), params__726.clone()).cast(std::sync::Arc::new(vec![t___11015.clone()])).validate_int64(csid__544("age"));
        let mut t___11019: bool = cs__727.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___38 {}
        impl ClosureGroup___38 {
            fn fn__11011(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___38 {};
        let fn__11011 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11011())
        };
        test___35.assert(t___11019, fn__11011.clone());
        test___35.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_failsForNonInteger__1657() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___36 = temper_std::testing::Test::new();
        let params__729: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___11002: TableDef = userTable__545();
        let mut t___11003: SafeIdentifier = csid__544("age");
        let cs__730: Changeset = changeset(t___11002.clone(), params__729.clone()).cast(std::sync::Arc::new(vec![t___11003.clone()])).validate_int64(csid__544("age"));
        let mut t___11009: bool = ! cs__730.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___39 {}
        impl ClosureGroup___39 {
            fn fn__10999(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___39 {};
        let fn__10999 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10999())
        };
        test___36.assert(t___11009, fn__10999.clone());
        test___36.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsTrue1_yesOn__1658() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___37 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___40 {
            test___37: temper_std::testing::Test
        }
        impl ClosureGroup___40 {
            fn fn__10996(& self, v__732: impl temper_core::ToArcString) {
                let v__732 = v__732.to_arc_string();
                let params__733: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__732.clone())]);
                let mut t___10988: TableDef = userTable__545();
                let mut t___10989: SafeIdentifier = csid__544("active");
                let cs__734: Changeset = changeset(t___10988.clone(), params__733.clone()).cast(std::sync::Arc::new(vec![t___10989.clone()])).validate_bool(csid__544("active"));
                let mut t___10993: bool = cs__734.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___41 {
                    v__732: std::sync::Arc<String>
                }
                impl ClosureGroup___41 {
                    fn fn__10985(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__732.clone()));
                    }
                }
                let closure_group = ClosureGroup___41 {
                    v__732: v__732.clone()
                };
                let fn__10985 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__10985())
                };
                self.test___37.assert(t___10993, fn__10985.clone());
            }
        }
        let closure_group = ClosureGroup___40 {
            test___37: test___37.clone()
        };
        let fn__10996 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__732: std::sync::Arc<String> | closure_group.fn__10996(v__732))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("true".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("yes".to_string()), std::sync::Arc::new("on".to_string())]), & ( * fn__10996.clone()));
        test___37.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsFalse0_noOff__1659() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___38 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___42 {
            test___38: temper_std::testing::Test
        }
        impl ClosureGroup___42 {
            fn fn__10982(& self, v__736: impl temper_core::ToArcString) {
                let v__736 = v__736.to_arc_string();
                let params__737: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__736.clone())]);
                let mut t___10974: TableDef = userTable__545();
                let mut t___10975: SafeIdentifier = csid__544("active");
                let cs__738: Changeset = changeset(t___10974.clone(), params__737.clone()).cast(std::sync::Arc::new(vec![t___10975.clone()])).validate_bool(csid__544("active"));
                let mut t___10979: bool = cs__738.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___43 {
                    v__736: std::sync::Arc<String>
                }
                impl ClosureGroup___43 {
                    fn fn__10971(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__736.clone()));
                    }
                }
                let closure_group = ClosureGroup___43 {
                    v__736: v__736.clone()
                };
                let fn__10971 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__10971())
                };
                self.test___38.assert(t___10979, fn__10971.clone());
            }
        }
        let closure_group = ClosureGroup___42 {
            test___38: test___38.clone()
        };
        let fn__10982 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__736: std::sync::Arc<String> | closure_group.fn__10982(v__736))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("false".to_string()), std::sync::Arc::new("0".to_string()), std::sync::Arc::new("no".to_string()), std::sync::Arc::new("off".to_string())]), & ( * fn__10982.clone()));
        test___38.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolRejectsAmbiguousValues__1660() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___39 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___44 {
            test___39: temper_std::testing::Test
        }
        impl ClosureGroup___44 {
            fn fn__10968(& self, v__740: impl temper_core::ToArcString) {
                let v__740 = v__740.to_arc_string();
                let params__741: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__740.clone())]);
                let mut t___10959: TableDef = userTable__545();
                let mut t___10960: SafeIdentifier = csid__544("active");
                let cs__742: Changeset = changeset(t___10959.clone(), params__741.clone()).cast(std::sync::Arc::new(vec![t___10960.clone()])).validate_bool(csid__544("active"));
                let mut t___10966: bool = ! cs__742.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___45 {
                    v__740: std::sync::Arc<String>
                }
                impl ClosureGroup___45 {
                    fn fn__10956(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject ambiguous: {}", self.v__740.clone()));
                    }
                }
                let closure_group = ClosureGroup___45 {
                    v__740: v__740.clone()
                };
                let fn__10956 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__10956())
                };
                self.test___39.assert(t___10966, fn__10956.clone());
            }
        }
        let closure_group = ClosureGroup___44 {
            test___39: test___39.clone()
        };
        let fn__10968 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__740: std::sync::Arc<String> | closure_group.fn__10968(v__740))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("TRUE".to_string()), std::sync::Arc::new("Yes".to_string()), std::sync::Arc::new("maybe".to_string()), std::sync::Arc::new("2".to_string()), std::sync::Arc::new("enabled".to_string())]), & ( * fn__10968.clone()));
        test___39.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEscapesBobbyTables__1661() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___40 = temper_std::testing::Test::new();
        let mut t___6144: SqlFragment;
        let params__744: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Robert'); DROP TABLE users;--".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bobby@evil.com".to_string()))]);
        let mut t___10944: TableDef = userTable__545();
        let mut t___10945: SafeIdentifier = csid__544("name");
        let mut t___10946: SafeIdentifier = csid__544("email");
        let cs__745: Changeset = changeset(t___10944.clone(), params__744.clone()).cast(std::sync::Arc::new(vec![t___10945.clone(), t___10946.clone()])).validate_required(std::sync::Arc::new(vec![csid__544("name"), csid__544("email")]));
        let sqlFrag__746: SqlFragment;
        'ok___11496: {
            'orelse___2013: {
                t___6144 = match cs__745.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2013
                };
                sqlFrag__746 = t___6144.clone();
                break 'ok___11496;
            }
            sqlFrag__746 = panic!();
        }
        let s__747: std::sync::Arc<String> = sqlFrag__746.to_string();
        let mut t___10953: bool = temper_core::string::index_of( & s__747, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___46 {
            s__747: std::sync::Arc<String>
        }
        impl ClosureGroup___46 {
            fn fn__10940(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("single quote must be doubled: {}", self.s__747.clone()));
            }
        }
        let closure_group = ClosureGroup___46 {
            s__747: s__747.clone()
        };
        let fn__10940 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10940())
        };
        test___40.assert(t___10953, fn__10940.clone());
        test___40.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForStringField__1662() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___41 = temper_std::testing::Test::new();
        let mut t___6123: SqlFragment;
        let params__749: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___10924: TableDef = userTable__545();
        let mut t___10925: SafeIdentifier = csid__544("name");
        let mut t___10926: SafeIdentifier = csid__544("email");
        let cs__750: Changeset = changeset(t___10924.clone(), params__749.clone()).cast(std::sync::Arc::new(vec![t___10925.clone(), t___10926.clone()])).validate_required(std::sync::Arc::new(vec![csid__544("name"), csid__544("email")]));
        let sqlFrag__751: SqlFragment;
        'ok___11499: {
            'orelse___2014: {
                t___6123 = match cs__750.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2014
                };
                sqlFrag__751 = t___6123.clone();
                break 'ok___11499;
            }
            sqlFrag__751 = panic!();
        }
        let s__752: std::sync::Arc<String> = sqlFrag__751.to_string();
        let mut t___10933: bool = temper_core::string::index_of( & s__752, "INSERT INTO users", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___47 {
            s__752: std::sync::Arc<String>
        }
        impl ClosureGroup___47 {
            fn fn__10920(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has INSERT INTO: {}", self.s__752.clone()));
            }
        }
        let closure_group = ClosureGroup___47 {
            s__752: s__752.clone()
        };
        let fn__10920 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10920())
        };
        test___41.assert(t___10933, fn__10920.clone());
        let mut t___10937: bool = temper_core::string::index_of( & s__752, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___48 {
            s__752: std::sync::Arc<String>
        }
        impl ClosureGroup___48 {
            fn fn__10919(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has quoted name: {}", self.s__752.clone()));
            }
        }
        let closure_group = ClosureGroup___48 {
            s__752: s__752.clone()
        };
        let fn__10919 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10919())
        };
        test___41.assert(t___10937, fn__10919.clone());
        test___41.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForIntField__1663() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___42 = temper_std::testing::Test::new();
        let mut t___6106: SqlFragment;
        let params__754: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("b@example.com".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___10906: TableDef = userTable__545();
        let mut t___10907: SafeIdentifier = csid__544("name");
        let mut t___10908: SafeIdentifier = csid__544("email");
        let mut t___10909: SafeIdentifier = csid__544("age");
        let cs__755: Changeset = changeset(t___10906.clone(), params__754.clone()).cast(std::sync::Arc::new(vec![t___10907.clone(), t___10908.clone(), t___10909.clone()])).validate_required(std::sync::Arc::new(vec![csid__544("name"), csid__544("email")]));
        let sqlFrag__756: SqlFragment;
        'ok___11500: {
            'orelse___2015: {
                t___6106 = match cs__755.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2015
                };
                sqlFrag__756 = t___6106.clone();
                break 'ok___11500;
            }
            sqlFrag__756 = panic!();
        }
        let s__757: std::sync::Arc<String> = sqlFrag__756.to_string();
        let mut t___10916: bool = temper_core::string::index_of( & s__757, "25", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___49 {
            s__757: std::sync::Arc<String>
        }
        impl ClosureGroup___49 {
            fn fn__10901(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("age rendered unquoted: {}", self.s__757.clone()));
            }
        }
        let closure_group = ClosureGroup___49 {
            s__757: s__757.clone()
        };
        let fn__10901 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10901())
        };
        test___42.assert(t___10916, fn__10901.clone());
        test___42.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlBubblesOnInvalidChangeset__1664() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___43 = temper_std::testing::Test::new();
        let params__759: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___10894: TableDef = userTable__545();
        let mut t___10895: SafeIdentifier = csid__544("name");
        let cs__760: Changeset = changeset(t___10894.clone(), params__759.clone()).cast(std::sync::Arc::new(vec![t___10895.clone()])).validate_required(std::sync::Arc::new(vec![csid__544("name")]));
        let didBubble__761: bool;
        'ok___11501: {
            'orelse___2016: {
                match cs__760.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2016
                };
                didBubble__761 = false;
                break 'ok___11501;
            }
            didBubble__761 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___50 {}
        impl ClosureGroup___50 {
            fn fn__10892(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___50 {};
        let fn__10892 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10892())
        };
        test___43.assert(didBubble__761, fn__10892.clone());
        test___43.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__1665() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___44 = temper_std::testing::Test::new();
        let strictTable__763: TableDef = TableDef::new(csid__544("posts"), [FieldDef::new(csid__544("title"), FieldType::new(StringField::new()), false), FieldDef::new(csid__544("body"), FieldType::new(StringField::new()), true)]);
        let params__764: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("body".to_string()), std::sync::Arc::new("hello".to_string()))]);
        let mut t___10885: SafeIdentifier = csid__544("body");
        let cs__765: Changeset = changeset(strictTable__763.clone(), params__764.clone()).cast(std::sync::Arc::new(vec![t___10885.clone()]));
        let mut t___10887: bool = cs__765.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___51 {}
        impl ClosureGroup___51 {
            fn fn__10874(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("changeset should appear valid (no explicit validation run)".to_string());
            }
        }
        let closure_group = ClosureGroup___51 {};
        let fn__10874 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10874())
        };
        test___44.assert(t___10887, fn__10874.clone());
        let didBubble__766: bool;
        'ok___11502: {
            'orelse___2017: {
                match cs__765.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2017
                };
                didBubble__766 = false;
                break 'ok___11502;
            }
            didBubble__766 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___52 {}
        impl ClosureGroup___52 {
            fn fn__10873(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("toInsertSql should enforce nullable regardless of isValid".to_string());
            }
        }
        let closure_group = ClosureGroup___52 {};
        let fn__10873 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10873())
        };
        test___44.assert(didBubble__766, fn__10873.clone());
        test___44.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlProducesCorrectSql__1666() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___45 = temper_std::testing::Test::new();
        let mut t___6066: SqlFragment;
        let params__768: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string()))]);
        let mut t___10864: TableDef = userTable__545();
        let mut t___10865: SafeIdentifier = csid__544("name");
        let cs__769: Changeset = changeset(t___10864.clone(), params__768.clone()).cast(std::sync::Arc::new(vec![t___10865.clone()])).validate_required(std::sync::Arc::new(vec![csid__544("name")]));
        let sqlFrag__770: SqlFragment;
        'ok___11503: {
            'orelse___2018: {
                t___6066 = match cs__769.to_update_sql(42) {
                    Ok(x) => x,
                    _ => break 'orelse___2018
                };
                sqlFrag__770 = t___6066.clone();
                break 'ok___11503;
            }
            sqlFrag__770 = panic!();
        }
        let s__771: std::sync::Arc<String> = sqlFrag__770.to_string();
        let mut t___10871: bool = Some(s__771.as_str()) == Some("UPDATE users SET name = 'Bob' WHERE id = 42");
        #[derive(Clone)]
        struct ClosureGroup___53 {
            s__771: std::sync::Arc<String>
        }
        impl ClosureGroup___53 {
            fn fn__10861(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__771.clone()));
            }
        }
        let closure_group = ClosureGroup___53 {
            s__771: s__771.clone()
        };
        let fn__10861 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10861())
        };
        test___45.assert(t___10871, fn__10861.clone());
        test___45.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlBubblesOnInvalidChangeset__1667() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___46 = temper_std::testing::Test::new();
        let params__773: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___10854: TableDef = userTable__545();
        let mut t___10855: SafeIdentifier = csid__544("name");
        let cs__774: Changeset = changeset(t___10854.clone(), params__773.clone()).cast(std::sync::Arc::new(vec![t___10855.clone()])).validate_required(std::sync::Arc::new(vec![csid__544("name")]));
        let didBubble__775: bool;
        'ok___11504: {
            'orelse___2019: {
                match cs__774.to_update_sql(1) {
                    Ok(x) => x,
                    _ => break 'orelse___2019
                };
                didBubble__775 = false;
                break 'ok___11504;
            }
            didBubble__775 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___54 {}
        impl ClosureGroup___54 {
            fn fn__10852(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___54 {};
        let fn__10852 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10852())
        };
        test___46.assert(didBubble__775, fn__10852.clone());
        test___46.soft_fail_to_hard()
    }
    #[test]
    fn bareFromProducesSelect__1749() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___47 = temper_std::testing::Test::new();
        let q__1123: Query = from(sid__546("users"));
        let mut t___10337: bool = Some(q__1123.to_sql().to_string().as_str()) == Some("SELECT * FROM users");
        #[derive(Clone)]
        struct ClosureGroup___55 {}
        impl ClosureGroup___55 {
            fn fn__10332(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("bare query".to_string());
            }
        }
        let closure_group = ClosureGroup___55 {};
        let fn__10332 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10332())
        };
        test___47.assert(t___10337, fn__10332.clone());
        test___47.soft_fail_to_hard()
    }
    #[test]
    fn selectRestrictsColumns__1750() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___48 = temper_std::testing::Test::new();
        let mut t___10323: SafeIdentifier = sid__546("users");
        let mut t___10324: SafeIdentifier = sid__546("id");
        let mut t___10325: SafeIdentifier = sid__546("name");
        let q__1125: Query = from(t___10323.clone()).select([t___10324.clone(), t___10325.clone()]);
        let mut t___10330: bool = Some(q__1125.to_sql().to_string().as_str()) == Some("SELECT id, name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___56 {}
        impl ClosureGroup___56 {
            fn fn__10322(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("select columns".to_string());
            }
        }
        let closure_group = ClosureGroup___56 {};
        let fn__10322 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10322())
        };
        test___48.assert(t___10330, fn__10322.clone());
        test___48.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithIntValue__1751() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___49 = temper_std::testing::Test::new();
        let mut t___10311: SafeIdentifier = sid__546("users");
        let mut t___10312: SqlBuilder = SqlBuilder::new();
        t___10312.append_safe("age > ");
        t___10312.append_int32(18);
        let mut t___10315: SqlFragment = t___10312.accumulated();
        let q__1127: Query = from(t___10311.clone()).r#where(t___10315.clone());
        let mut t___10320: bool = Some(q__1127.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18");
        #[derive(Clone)]
        struct ClosureGroup___57 {}
        impl ClosureGroup___57 {
            fn fn__10310(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where int".to_string());
            }
        }
        let closure_group = ClosureGroup___57 {};
        let fn__10310 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10310())
        };
        test___49.assert(t___10320, fn__10310.clone());
        test___49.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithBoolValue__1753() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___50 = temper_std::testing::Test::new();
        let mut t___10299: SafeIdentifier = sid__546("users");
        let mut t___10300: SqlBuilder = SqlBuilder::new();
        t___10300.append_safe("active = ");
        t___10300.append_boolean(true);
        let mut t___10303: SqlFragment = t___10300.accumulated();
        let q__1129: Query = from(t___10299.clone()).r#where(t___10303.clone());
        let mut t___10308: bool = Some(q__1129.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___58 {}
        impl ClosureGroup___58 {
            fn fn__10298(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where bool".to_string());
            }
        }
        let closure_group = ClosureGroup___58 {};
        let fn__10298 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10298())
        };
        test___50.assert(t___10308, fn__10298.clone());
        test___50.soft_fail_to_hard()
    }
    #[test]
    fn chainedWhereUsesAnd__1755() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___51 = temper_std::testing::Test::new();
        let mut t___10282: SafeIdentifier = sid__546("users");
        let mut t___10283: SqlBuilder = SqlBuilder::new();
        t___10283.append_safe("age > ");
        t___10283.append_int32(18);
        let mut t___10286: SqlFragment = t___10283.accumulated();
        let mut t___10287: Query = from(t___10282.clone()).r#where(t___10286.clone());
        let mut t___10288: SqlBuilder = SqlBuilder::new();
        t___10288.append_safe("active = ");
        t___10288.append_boolean(true);
        let q__1131: Query = t___10287.r#where(t___10288.accumulated());
        let mut t___10296: bool = Some(q__1131.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___59 {}
        impl ClosureGroup___59 {
            fn fn__10281(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained where".to_string());
            }
        }
        let closure_group = ClosureGroup___59 {};
        let fn__10281 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10281())
        };
        test___51.assert(t___10296, fn__10281.clone());
        test___51.soft_fail_to_hard()
    }
    #[test]
    fn orderByAsc__1758() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___52 = temper_std::testing::Test::new();
        let mut t___10273: SafeIdentifier = sid__546("users");
        let mut t___10274: SafeIdentifier = sid__546("name");
        let q__1133: Query = from(t___10273.clone()).order_by(t___10274.clone(), true);
        let mut t___10279: bool = Some(q__1133.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC");
        #[derive(Clone)]
        struct ClosureGroup___60 {}
        impl ClosureGroup___60 {
            fn fn__10272(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order asc".to_string());
            }
        }
        let closure_group = ClosureGroup___60 {};
        let fn__10272 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10272())
        };
        test___52.assert(t___10279, fn__10272.clone());
        test___52.soft_fail_to_hard()
    }
    #[test]
    fn orderByDesc__1759() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___53 = temper_std::testing::Test::new();
        let mut t___10264: SafeIdentifier = sid__546("users");
        let mut t___10265: SafeIdentifier = sid__546("created_at");
        let q__1135: Query = from(t___10264.clone()).order_by(t___10265.clone(), false);
        let mut t___10270: bool = Some(q__1135.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY created_at DESC");
        #[derive(Clone)]
        struct ClosureGroup___61 {}
        impl ClosureGroup___61 {
            fn fn__10263(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order desc".to_string());
            }
        }
        let closure_group = ClosureGroup___61 {};
        let fn__10263 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10263())
        };
        test___53.assert(t___10270, fn__10263.clone());
        test___53.soft_fail_to_hard()
    }
    #[test]
    fn limitAndOffset__1760() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___54 = temper_std::testing::Test::new();
        let mut t___5460: Query;
        let mut t___5461: Query;
        let q__1137: Query;
        'ok___11506: {
            'orelse___2021: {
                t___5460 = match from(sid__546("users")).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___2021
                };
                t___5461 = match t___5460.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___2021
                };
                q__1137 = t___5461.clone();
                break 'ok___11506;
            }
            q__1137 = panic!();
        }
        let mut t___10261: bool = Some(q__1137.to_sql().to_string().as_str()) == Some("SELECT * FROM users LIMIT 10 OFFSET 20");
        #[derive(Clone)]
        struct ClosureGroup___62 {}
        impl ClosureGroup___62 {
            fn fn__10256(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("limit/offset".to_string());
            }
        }
        let closure_group = ClosureGroup___62 {};
        let fn__10256 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10256())
        };
        test___54.assert(t___10261, fn__10256.clone());
        test___54.soft_fail_to_hard()
    }
    #[test]
    fn limitBubblesOnNegative__1761() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___55 = temper_std::testing::Test::new();
        let didBubble__1139: bool;
        'ok___11507: {
            'orelse___2022: {
                match from(sid__546("users")).limit(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___2022
                };
                didBubble__1139 = false;
                break 'ok___11507;
            }
            didBubble__1139 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___63 {}
        impl ClosureGroup___63 {
            fn fn__10252(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative limit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___63 {};
        let fn__10252 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10252())
        };
        test___55.assert(didBubble__1139, fn__10252.clone());
        test___55.soft_fail_to_hard()
    }
    #[test]
    fn offsetBubblesOnNegative__1762() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___56 = temper_std::testing::Test::new();
        let didBubble__1141: bool;
        'ok___11508: {
            'orelse___2023: {
                match from(sid__546("users")).offset(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___2023
                };
                didBubble__1141 = false;
                break 'ok___11508;
            }
            didBubble__1141 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___64 {}
        impl ClosureGroup___64 {
            fn fn__10248(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative offset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___64 {};
        let fn__10248 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10248())
        };
        test___56.assert(didBubble__1141, fn__10248.clone());
        test___56.soft_fail_to_hard()
    }
    #[test]
    fn complexComposedQuery__1763() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___57 = temper_std::testing::Test::new();
        let mut t___10226: SafeIdentifier;
        let mut t___10227: SafeIdentifier;
        let mut t___10228: SafeIdentifier;
        let mut t___10229: SafeIdentifier;
        let mut t___10230: Query;
        let mut t___10231: SqlBuilder;
        let mut t___10235: Query;
        let mut t___10236: SqlBuilder;
        let mut t___5446: Query;
        let mut t___5447: Query;
        let minAge__1143: i32 = 21;
        let q__1144: Query;
        'ok___11509: {
            'orelse___2024: {
                t___10226 = sid__546("users");
                t___10227 = sid__546("id");
                t___10228 = sid__546("name");
                t___10229 = sid__546("email");
                t___10230 = from(t___10226.clone()).select([t___10227.clone(), t___10228.clone(), t___10229.clone()]);
                t___10231 = SqlBuilder::new();
                t___10231.append_safe("age >= ");
                t___10231.append_int32(21);
                t___10235 = t___10230.r#where(t___10231.accumulated());
                t___10236 = SqlBuilder::new();
                t___10236.append_safe("active = ");
                t___10236.append_boolean(true);
                t___5446 = match t___10235.r#where(t___10236.accumulated()).order_by(sid__546("name"), true).limit(25) {
                    Ok(x) => x,
                    _ => break 'orelse___2024
                };
                t___5447 = match t___5446.offset(0) {
                    Ok(x) => x,
                    _ => break 'orelse___2024
                };
                q__1144 = t___5447.clone();
                break 'ok___11509;
            }
            q__1144 = panic!();
        }
        let mut t___10246: bool = Some(q__1144.to_sql().to_string().as_str()) == Some("SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0");
        #[derive(Clone)]
        struct ClosureGroup___65 {}
        impl ClosureGroup___65 {
            fn fn__10225(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("complex query".to_string());
            }
        }
        let closure_group = ClosureGroup___65 {};
        let fn__10225 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10225())
        };
        test___57.assert(t___10246, fn__10225.clone());
        test___57.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlAppliesDefaultLimitWhenNoneSet__1766() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___58 = temper_std::testing::Test::new();
        let mut t___5423: SqlFragment;
        let mut t___5424: SqlFragment;
        let q__1146: Query = from(sid__546("users"));
        'ok___11510: {
            'orelse___2025: {
                t___5423 = match q__1146.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___2025
                };
                t___5424 = t___5423.clone();
                break 'ok___11510;
            }
            t___5424 = panic!();
        }
        let s__1147: std::sync::Arc<String> = t___5424.to_string();
        let mut t___10223: bool = Some(s__1147.as_str()) == Some("SELECT * FROM users LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___66 {
            s__1147: std::sync::Arc<String>
        }
        impl ClosureGroup___66 {
            fn fn__10219(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should have limit: {}", self.s__1147.clone()));
            }
        }
        let closure_group = ClosureGroup___66 {
            s__1147: s__1147.clone()
        };
        let fn__10219 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10219())
        };
        test___58.assert(t___10223, fn__10219.clone());
        test___58.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlRespectsExplicitLimit__1767() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___59 = temper_std::testing::Test::new();
        let mut t___5415: Query;
        let mut t___5418: SqlFragment;
        let mut t___5419: SqlFragment;
        let q__1149: Query;
        'ok___11511: {
            'orelse___2026: {
                t___5415 = match from(sid__546("users")).limit(5) {
                    Ok(x) => x,
                    _ => break 'orelse___2026
                };
                q__1149 = t___5415.clone();
                break 'ok___11511;
            }
            q__1149 = panic!();
        }
        'ok___11512: {
            'orelse___2027: {
                t___5418 = match q__1149.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___2027
                };
                t___5419 = t___5418.clone();
                break 'ok___11512;
            }
            t___5419 = panic!();
        }
        let s__1150: std::sync::Arc<String> = t___5419.to_string();
        let mut t___10217: bool = Some(s__1150.as_str()) == Some("SELECT * FROM users LIMIT 5");
        #[derive(Clone)]
        struct ClosureGroup___67 {
            s__1150: std::sync::Arc<String>
        }
        impl ClosureGroup___67 {
            fn fn__10213(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("explicit limit preserved: {}", self.s__1150.clone()));
            }
        }
        let closure_group = ClosureGroup___67 {
            s__1150: s__1150.clone()
        };
        let fn__10213 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10213())
        };
        test___59.assert(t___10217, fn__10213.clone());
        test___59.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlBubblesOnNegativeDefaultLimit__1768() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___60 = temper_std::testing::Test::new();
        let didBubble__1152: bool;
        'ok___11513: {
            'orelse___2028: {
                match from(sid__546("users")).safe_to_sql(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___2028
                };
                didBubble__1152 = false;
                break 'ok___11513;
            }
            didBubble__1152 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___68 {}
        impl ClosureGroup___68 {
            fn fn__10209(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative defaultLimit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___68 {};
        let fn__10209 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10209())
        };
        test___60.assert(didBubble__1152, fn__10209.clone());
        test___60.soft_fail_to_hard()
    }
    #[test]
    fn whereWithInjectionAttemptInStringValueIsEscaped__1769() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___61 = temper_std::testing::Test::new();
        let evil__1154: std::sync::Arc<String> = std::sync::Arc::new("'; DROP TABLE users; --".to_string());
        let mut t___10193: SafeIdentifier = sid__546("users");
        let mut t___10194: SqlBuilder = SqlBuilder::new();
        t___10194.append_safe("name = ");
        t___10194.append_string("'; DROP TABLE users; --");
        let mut t___10197: SqlFragment = t___10194.accumulated();
        let q__1155: Query = from(t___10193.clone()).r#where(t___10197.clone());
        let s__1156: std::sync::Arc<String> = q__1155.to_sql().to_string();
        let mut t___10202: bool = temper_core::string::index_of( & s__1156, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___69 {
            s__1156: std::sync::Arc<String>
        }
        impl ClosureGroup___69 {
            fn fn__10192(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("quotes must be doubled: {}", self.s__1156.clone()));
            }
        }
        let closure_group = ClosureGroup___69 {
            s__1156: s__1156.clone()
        };
        let fn__10192 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10192())
        };
        test___61.assert(t___10202, fn__10192.clone());
        let mut t___10206: bool = temper_core::string::index_of( & s__1156, "SELECT * FROM users WHERE name =", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___70 {
            s__1156: std::sync::Arc<String>
        }
        impl ClosureGroup___70 {
            fn fn__10191(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("structure intact: {}", self.s__1156.clone()));
            }
        }
        let closure_group = ClosureGroup___70 {
            s__1156: s__1156.clone()
        };
        let fn__10191 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10191())
        };
        test___61.assert(t___10206, fn__10191.clone());
        test___61.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__1771() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___62 = temper_std::testing::Test::new();
        let attack__1158: std::sync::Arc<String> = std::sync::Arc::new("users; DROP TABLE users; --".to_string());
        let didBubble__1159: bool;
        'ok___11514: {
            'orelse___2029: {
                match safe_identifier("users; DROP TABLE users; --") {
                    Ok(x) => x,
                    _ => break 'orelse___2029
                };
                didBubble__1159 = false;
                break 'ok___11514;
            }
            didBubble__1159 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___71 {}
        impl ClosureGroup___71 {
            fn fn__10188(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("metacharacter-containing name must be rejected at construction".to_string());
            }
        }
        let closure_group = ClosureGroup___71 {};
        let fn__10188 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10188())
        };
        test___62.assert(didBubble__1159, fn__10188.clone());
        test___62.soft_fail_to_hard()
    }
    #[test]
    fn innerJoinProducesInnerJoin__1772() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___63 = temper_std::testing::Test::new();
        let mut t___10177: SafeIdentifier = sid__546("users");
        let mut t___10178: SafeIdentifier = sid__546("orders");
        let mut t___10179: SqlBuilder = SqlBuilder::new();
        t___10179.append_safe("users.id = orders.user_id");
        let mut t___10181: SqlFragment = t___10179.accumulated();
        let q__1161: Query = from(t___10177.clone()).inner_join(t___10178.clone(), t___10181.clone());
        let mut t___10186: bool = Some(q__1161.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___72 {}
        impl ClosureGroup___72 {
            fn fn__10176(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("inner join".to_string());
            }
        }
        let closure_group = ClosureGroup___72 {};
        let fn__10176 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10176())
        };
        test___63.assert(t___10186, fn__10176.clone());
        test___63.soft_fail_to_hard()
    }
    #[test]
    fn leftJoinProducesLeftJoin__1774() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___64 = temper_std::testing::Test::new();
        let mut t___10165: SafeIdentifier = sid__546("users");
        let mut t___10166: SafeIdentifier = sid__546("profiles");
        let mut t___10167: SqlBuilder = SqlBuilder::new();
        t___10167.append_safe("users.id = profiles.user_id");
        let mut t___10169: SqlFragment = t___10167.accumulated();
        let q__1163: Query = from(t___10165.clone()).left_join(t___10166.clone(), t___10169.clone());
        let mut t___10174: bool = Some(q__1163.to_sql().to_string().as_str()) == Some("SELECT * FROM users LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___73 {}
        impl ClosureGroup___73 {
            fn fn__10164(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("left join".to_string());
            }
        }
        let closure_group = ClosureGroup___73 {};
        let fn__10164 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10164())
        };
        test___64.assert(t___10174, fn__10164.clone());
        test___64.soft_fail_to_hard()
    }
    #[test]
    fn rightJoinProducesRightJoin__1776() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___65 = temper_std::testing::Test::new();
        let mut t___10153: SafeIdentifier = sid__546("orders");
        let mut t___10154: SafeIdentifier = sid__546("users");
        let mut t___10155: SqlBuilder = SqlBuilder::new();
        t___10155.append_safe("orders.user_id = users.id");
        let mut t___10157: SqlFragment = t___10155.accumulated();
        let q__1165: Query = from(t___10153.clone()).right_join(t___10154.clone(), t___10157.clone());
        let mut t___10162: bool = Some(q__1165.to_sql().to_string().as_str()) == Some("SELECT * FROM orders RIGHT JOIN users ON orders.user_id = users.id");
        #[derive(Clone)]
        struct ClosureGroup___74 {}
        impl ClosureGroup___74 {
            fn fn__10152(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("right join".to_string());
            }
        }
        let closure_group = ClosureGroup___74 {};
        let fn__10152 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10152())
        };
        test___65.assert(t___10162, fn__10152.clone());
        test___65.soft_fail_to_hard()
    }
    #[test]
    fn fullJoinProducesFullOuterJoin__1778() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___66 = temper_std::testing::Test::new();
        let mut t___10141: SafeIdentifier = sid__546("users");
        let mut t___10142: SafeIdentifier = sid__546("orders");
        let mut t___10143: SqlBuilder = SqlBuilder::new();
        t___10143.append_safe("users.id = orders.user_id");
        let mut t___10145: SqlFragment = t___10143.accumulated();
        let q__1167: Query = from(t___10141.clone()).full_join(t___10142.clone(), t___10145.clone());
        let mut t___10150: bool = Some(q__1167.to_sql().to_string().as_str()) == Some("SELECT * FROM users FULL OUTER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___75 {}
        impl ClosureGroup___75 {
            fn fn__10140(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full join".to_string());
            }
        }
        let closure_group = ClosureGroup___75 {};
        let fn__10140 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10140())
        };
        test___66.assert(t___10150, fn__10140.clone());
        test___66.soft_fail_to_hard()
    }
    #[test]
    fn chainedJoins__1780() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___67 = temper_std::testing::Test::new();
        let mut t___10124: SafeIdentifier = sid__546("users");
        let mut t___10125: SafeIdentifier = sid__546("orders");
        let mut t___10126: SqlBuilder = SqlBuilder::new();
        t___10126.append_safe("users.id = orders.user_id");
        let mut t___10128: SqlFragment = t___10126.accumulated();
        let mut t___10129: Query = from(t___10124.clone()).inner_join(t___10125.clone(), t___10128.clone());
        let mut t___10130: SafeIdentifier = sid__546("profiles");
        let mut t___10131: SqlBuilder = SqlBuilder::new();
        t___10131.append_safe("users.id = profiles.user_id");
        let q__1169: Query = t___10129.left_join(t___10130.clone(), t___10131.accumulated());
        let mut t___10138: bool = Some(q__1169.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___76 {}
        impl ClosureGroup___76 {
            fn fn__10123(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained joins".to_string());
            }
        }
        let closure_group = ClosureGroup___76 {};
        let fn__10123 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10123())
        };
        test___67.assert(t___10138, fn__10123.clone());
        test___67.soft_fail_to_hard()
    }
    #[test]
    fn joinWithWhereAndOrderBy__1783() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___68 = temper_std::testing::Test::new();
        let mut t___10105: SafeIdentifier;
        let mut t___10106: SafeIdentifier;
        let mut t___10107: SqlBuilder;
        let mut t___10109: SqlFragment;
        let mut t___10110: Query;
        let mut t___10111: SqlBuilder;
        let mut t___5330: Query;
        let q__1171: Query;
        'ok___11515: {
            'orelse___2030: {
                t___10105 = sid__546("users");
                t___10106 = sid__546("orders");
                t___10107 = SqlBuilder::new();
                t___10107.append_safe("users.id = orders.user_id");
                t___10109 = t___10107.accumulated();
                t___10110 = from(t___10105.clone()).inner_join(t___10106.clone(), t___10109.clone());
                t___10111 = SqlBuilder::new();
                t___10111.append_safe("orders.total > ");
                t___10111.append_int32(100);
                t___5330 = match t___10110.r#where(t___10111.accumulated()).order_by(sid__546("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___2030
                };
                q__1171 = t___5330.clone();
                break 'ok___11515;
            }
            q__1171 = panic!();
        }
        let mut t___10121: bool = Some(q__1171.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100 ORDER BY name ASC LIMIT 10");
        #[derive(Clone)]
        struct ClosureGroup___77 {}
        impl ClosureGroup___77 {
            fn fn__10104(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with where/order/limit".to_string());
            }
        }
        let closure_group = ClosureGroup___77 {};
        let fn__10104 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10104())
        };
        test___68.assert(t___10121, fn__10104.clone());
        test___68.soft_fail_to_hard()
    }
    #[test]
    fn colHelperProducesQualifiedReference__1786() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___69 = temper_std::testing::Test::new();
        let c__1173: SqlFragment = col(sid__546("users"), sid__546("id"));
        let mut t___10102: bool = Some(c__1173.to_string().as_str()) == Some("users.id");
        #[derive(Clone)]
        struct ClosureGroup___78 {}
        impl ClosureGroup___78 {
            fn fn__10096(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("col helper".to_string());
            }
        }
        let closure_group = ClosureGroup___78 {};
        let fn__10096 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10096())
        };
        test___69.assert(t___10102, fn__10096.clone());
        test___69.soft_fail_to_hard()
    }
    #[test]
    fn joinWithColHelper__1787() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___70 = temper_std::testing::Test::new();
        let onCond__1175: SqlFragment = col(sid__546("users"), sid__546("id"));
        let b__1176: SqlBuilder = SqlBuilder::new();
        b__1176.append_fragment(onCond__1175.clone());
        b__1176.append_safe(" = ");
        b__1176.append_fragment(col(sid__546("orders"), sid__546("user_id")));
        let mut t___10087: SafeIdentifier = sid__546("users");
        let mut t___10088: SafeIdentifier = sid__546("orders");
        let mut t___10089: SqlFragment = b__1176.accumulated();
        let q__1177: Query = from(t___10087.clone()).inner_join(t___10088.clone(), t___10089.clone());
        let mut t___10094: bool = Some(q__1177.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___79 {}
        impl ClosureGroup___79 {
            fn fn__10076(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with col".to_string());
            }
        }
        let closure_group = ClosureGroup___79 {};
        let fn__10076 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10076())
        };
        test___70.assert(t___10094, fn__10076.clone());
        test___70.soft_fail_to_hard()
    }
    #[test]
    fn orWhereBasic__1788() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___71 = temper_std::testing::Test::new();
        let mut t___10065: SafeIdentifier = sid__546("users");
        let mut t___10066: SqlBuilder = SqlBuilder::new();
        t___10066.append_safe("status = ");
        t___10066.append_string("active");
        let mut t___10069: SqlFragment = t___10066.accumulated();
        let q__1179: Query = from(t___10065.clone()).or_where(t___10069.clone());
        let mut t___10074: bool = Some(q__1179.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE status = 'active'");
        #[derive(Clone)]
        struct ClosureGroup___80 {}
        impl ClosureGroup___80 {
            fn fn__10064(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orWhere basic".to_string());
            }
        }
        let closure_group = ClosureGroup___80 {};
        let fn__10064 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10064())
        };
        test___71.assert(t___10074, fn__10064.clone());
        test___71.soft_fail_to_hard()
    }
    #[test]
    fn whereThenOrWhere__1790() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___72 = temper_std::testing::Test::new();
        let mut t___10048: SafeIdentifier = sid__546("users");
        let mut t___10049: SqlBuilder = SqlBuilder::new();
        t___10049.append_safe("age > ");
        t___10049.append_int32(18);
        let mut t___10052: SqlFragment = t___10049.accumulated();
        let mut t___10053: Query = from(t___10048.clone()).r#where(t___10052.clone());
        let mut t___10054: SqlBuilder = SqlBuilder::new();
        t___10054.append_safe("vip = ");
        t___10054.append_boolean(true);
        let q__1181: Query = t___10053.or_where(t___10054.accumulated());
        let mut t___10062: bool = Some(q__1181.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___81 {}
        impl ClosureGroup___81 {
            fn fn__10047(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where then orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___81 {};
        let fn__10047 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10047())
        };
        test___72.assert(t___10062, fn__10047.clone());
        test___72.soft_fail_to_hard()
    }
    #[test]
    fn multipleOrWhere__1793() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___73 = temper_std::testing::Test::new();
        let mut t___10026: SafeIdentifier = sid__546("users");
        let mut t___10027: SqlBuilder = SqlBuilder::new();
        t___10027.append_safe("active = ");
        t___10027.append_boolean(true);
        let mut t___10030: SqlFragment = t___10027.accumulated();
        let mut t___10031: Query = from(t___10026.clone()).r#where(t___10030.clone());
        let mut t___10032: SqlBuilder = SqlBuilder::new();
        t___10032.append_safe("role = ");
        t___10032.append_string("admin");
        let mut t___10036: Query = t___10031.or_where(t___10032.accumulated());
        let mut t___10037: SqlBuilder = SqlBuilder::new();
        t___10037.append_safe("role = ");
        t___10037.append_string("moderator");
        let q__1183: Query = t___10036.or_where(t___10037.accumulated());
        let mut t___10045: bool = Some(q__1183.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE OR role = 'admin' OR role = 'moderator'");
        #[derive(Clone)]
        struct ClosureGroup___82 {}
        impl ClosureGroup___82 {
            fn fn__10025(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("multiple orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___82 {};
        let fn__10025 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10025())
        };
        test___73.assert(t___10045, fn__10025.clone());
        test___73.soft_fail_to_hard()
    }
    #[test]
    fn mixedWhereAndOrWhere__1797() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___74 = temper_std::testing::Test::new();
        let mut t___10004: SafeIdentifier = sid__546("users");
        let mut t___10005: SqlBuilder = SqlBuilder::new();
        t___10005.append_safe("age > ");
        t___10005.append_int32(18);
        let mut t___10008: SqlFragment = t___10005.accumulated();
        let mut t___10009: Query = from(t___10004.clone()).r#where(t___10008.clone());
        let mut t___10010: SqlBuilder = SqlBuilder::new();
        t___10010.append_safe("active = ");
        t___10010.append_boolean(true);
        let mut t___10014: Query = t___10009.r#where(t___10010.accumulated());
        let mut t___10015: SqlBuilder = SqlBuilder::new();
        t___10015.append_safe("vip = ");
        t___10015.append_boolean(true);
        let q__1185: Query = t___10014.or_where(t___10015.accumulated());
        let mut t___10023: bool = Some(q__1185.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___83 {}
        impl ClosureGroup___83 {
            fn fn__10003(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mixed where and orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___83 {};
        let fn__10003 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10003())
        };
        test___74.assert(t___10023, fn__10003.clone());
        test___74.soft_fail_to_hard()
    }
    #[test]
    fn whereNull__1801() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___75 = temper_std::testing::Test::new();
        let mut t___9995: SafeIdentifier = sid__546("users");
        let mut t___9996: SafeIdentifier = sid__546("deleted_at");
        let q__1187: Query = from(t___9995.clone()).where_null(t___9996.clone());
        let mut t___10001: bool = Some(q__1187.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___84 {}
        impl ClosureGroup___84 {
            fn fn__9994(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull".to_string());
            }
        }
        let closure_group = ClosureGroup___84 {};
        let fn__9994 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9994())
        };
        test___75.assert(t___10001, fn__9994.clone());
        test___75.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNull__1802() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___76 = temper_std::testing::Test::new();
        let mut t___9986: SafeIdentifier = sid__546("users");
        let mut t___9987: SafeIdentifier = sid__546("email");
        let q__1189: Query = from(t___9986.clone()).where_not_null(t___9987.clone());
        let mut t___9992: bool = Some(q__1189.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email IS NOT NULL");
        #[derive(Clone)]
        struct ClosureGroup___85 {}
        impl ClosureGroup___85 {
            fn fn__9985(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull".to_string());
            }
        }
        let closure_group = ClosureGroup___85 {};
        let fn__9985 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9985())
        };
        test___76.assert(t___9992, fn__9985.clone());
        test___76.soft_fail_to_hard()
    }
    #[test]
    fn whereNullChainedWithWhere__1803() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___77 = temper_std::testing::Test::new();
        let mut t___9972: SafeIdentifier = sid__546("users");
        let mut t___9973: SqlBuilder = SqlBuilder::new();
        t___9973.append_safe("active = ");
        t___9973.append_boolean(true);
        let mut t___9976: SqlFragment = t___9973.accumulated();
        let q__1191: Query = from(t___9972.clone()).r#where(t___9976.clone()).where_null(sid__546("deleted_at"));
        let mut t___9983: bool = Some(q__1191.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___86 {}
        impl ClosureGroup___86 {
            fn fn__9971(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull chained".to_string());
            }
        }
        let closure_group = ClosureGroup___86 {};
        let fn__9971 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9971())
        };
        test___77.assert(t___9983, fn__9971.clone());
        test___77.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNullChainedWithOrWhere__1805() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___78 = temper_std::testing::Test::new();
        let mut t___9958: SafeIdentifier = sid__546("users");
        let mut t___9959: SafeIdentifier = sid__546("deleted_at");
        let mut t___9960: Query = from(t___9958.clone()).where_null(t___9959.clone());
        let mut t___9961: SqlBuilder = SqlBuilder::new();
        t___9961.append_safe("role = ");
        t___9961.append_string("admin");
        let q__1193: Query = t___9960.or_where(t___9961.accumulated());
        let mut t___9969: bool = Some(q__1193.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL OR role = 'admin'");
        #[derive(Clone)]
        struct ClosureGroup___87 {}
        impl ClosureGroup___87 {
            fn fn__9957(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull with orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___87 {};
        let fn__9957 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9957())
        };
        test___78.assert(t___9969, fn__9957.clone());
        test___78.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithIntValues__1807() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___79 = temper_std::testing::Test::new();
        let mut t___9946: SafeIdentifier = sid__546("users");
        let mut t___9947: SafeIdentifier = sid__546("id");
        let mut t___9948: SqlInt32 = SqlInt32::new(1);
        let mut t___9949: SqlInt32 = SqlInt32::new(2);
        let mut t___9950: SqlInt32 = SqlInt32::new(3);
        let q__1195: Query = from(t___9946.clone()).where_in(t___9947.clone(), [SqlPart::new(t___9948.clone()), SqlPart::new(t___9949.clone()), SqlPart::new(t___9950.clone())]);
        let mut t___9955: bool = Some(q__1195.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___88 {}
        impl ClosureGroup___88 {
            fn fn__9945(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn ints".to_string());
            }
        }
        let closure_group = ClosureGroup___88 {};
        let fn__9945 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9945())
        };
        test___79.assert(t___9955, fn__9945.clone());
        test___79.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithStringValuesEscaping__1808() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___80 = temper_std::testing::Test::new();
        let mut t___9935: SafeIdentifier = sid__546("users");
        let mut t___9936: SafeIdentifier = sid__546("name");
        let mut t___9937: SqlString = SqlString::new("Alice");
        let mut t___9938: SqlString = SqlString::new("Bob's");
        let q__1197: Query = from(t___9935.clone()).where_in(t___9936.clone(), [SqlPart::new(t___9937.clone()), SqlPart::new(t___9938.clone())]);
        let mut t___9943: bool = Some(q__1197.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name IN ('Alice', 'Bob''s')");
        #[derive(Clone)]
        struct ClosureGroup___89 {}
        impl ClosureGroup___89 {
            fn fn__9934(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn strings".to_string());
            }
        }
        let closure_group = ClosureGroup___89 {};
        let fn__9934 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9934())
        };
        test___80.assert(t___9943, fn__9934.clone());
        test___80.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithEmptyListProduces1_0__1809() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___81 = temper_std::testing::Test::new();
        let mut t___9926: SafeIdentifier = sid__546("users");
        let mut t___9927: SafeIdentifier = sid__546("id");
        let q__1199: Query = from(t___9926.clone()).where_in(t___9927.clone(), []);
        let mut t___9932: bool = Some(q__1199.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE 1 = 0");
        #[derive(Clone)]
        struct ClosureGroup___90 {}
        impl ClosureGroup___90 {
            fn fn__9925(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn empty".to_string());
            }
        }
        let closure_group = ClosureGroup___90 {};
        let fn__9925 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9925())
        };
        test___81.assert(t___9932, fn__9925.clone());
        test___81.soft_fail_to_hard()
    }
    #[test]
    fn whereInChained__1810() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___82 = temper_std::testing::Test::new();
        let mut t___9910: SafeIdentifier = sid__546("users");
        let mut t___9911: SqlBuilder = SqlBuilder::new();
        t___9911.append_safe("active = ");
        t___9911.append_boolean(true);
        let mut t___9914: SqlFragment = t___9911.accumulated();
        let q__1201: Query = from(t___9910.clone()).r#where(t___9914.clone()).where_in(sid__546("role"), [SqlPart::new(SqlString::new("admin")), SqlPart::new(SqlString::new("user"))]);
        let mut t___9923: bool = Some(q__1201.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND role IN ('admin', 'user')");
        #[derive(Clone)]
        struct ClosureGroup___91 {}
        impl ClosureGroup___91 {
            fn fn__9909(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn chained".to_string());
            }
        }
        let closure_group = ClosureGroup___91 {};
        let fn__9909 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9909())
        };
        test___82.assert(t___9923, fn__9909.clone());
        test___82.soft_fail_to_hard()
    }
    #[test]
    fn whereInSingleElement__1812() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___83 = temper_std::testing::Test::new();
        let mut t___9900: SafeIdentifier = sid__546("users");
        let mut t___9901: SafeIdentifier = sid__546("id");
        let mut t___9902: SqlInt32 = SqlInt32::new(42);
        let q__1203: Query = from(t___9900.clone()).where_in(t___9901.clone(), [SqlPart::new(t___9902.clone())]);
        let mut t___9907: bool = Some(q__1203.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (42)");
        #[derive(Clone)]
        struct ClosureGroup___92 {}
        impl ClosureGroup___92 {
            fn fn__9899(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn single".to_string());
            }
        }
        let closure_group = ClosureGroup___92 {};
        let fn__9899 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9899())
        };
        test___83.assert(t___9907, fn__9899.clone());
        test___83.soft_fail_to_hard()
    }
    #[test]
    fn whereNotBasic__1813() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___84 = temper_std::testing::Test::new();
        let mut t___9888: SafeIdentifier = sid__546("users");
        let mut t___9889: SqlBuilder = SqlBuilder::new();
        t___9889.append_safe("active = ");
        t___9889.append_boolean(true);
        let mut t___9892: SqlFragment = t___9889.accumulated();
        let q__1205: Query = from(t___9888.clone()).where_not(t___9892.clone());
        let mut t___9897: bool = Some(q__1205.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE NOT (active = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___93 {}
        impl ClosureGroup___93 {
            fn fn__9887(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot".to_string());
            }
        }
        let closure_group = ClosureGroup___93 {};
        let fn__9887 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9887())
        };
        test___84.assert(t___9897, fn__9887.clone());
        test___84.soft_fail_to_hard()
    }
    #[test]
    fn whereNotChained__1815() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___85 = temper_std::testing::Test::new();
        let mut t___9871: SafeIdentifier = sid__546("users");
        let mut t___9872: SqlBuilder = SqlBuilder::new();
        t___9872.append_safe("age > ");
        t___9872.append_int32(18);
        let mut t___9875: SqlFragment = t___9872.accumulated();
        let mut t___9876: Query = from(t___9871.clone()).r#where(t___9875.clone());
        let mut t___9877: SqlBuilder = SqlBuilder::new();
        t___9877.append_safe("banned = ");
        t___9877.append_boolean(true);
        let q__1207: Query = t___9876.where_not(t___9877.accumulated());
        let mut t___9885: bool = Some(q__1207.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND NOT (banned = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___94 {}
        impl ClosureGroup___94 {
            fn fn__9870(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot chained".to_string());
            }
        }
        let closure_group = ClosureGroup___94 {};
        let fn__9870 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9870())
        };
        test___85.assert(t___9885, fn__9870.clone());
        test___85.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenIntegers__1818() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___86 = temper_std::testing::Test::new();
        let mut t___9860: SafeIdentifier = sid__546("users");
        let mut t___9861: SafeIdentifier = sid__546("age");
        let mut t___9862: SqlInt32 = SqlInt32::new(18);
        let mut t___9863: SqlInt32 = SqlInt32::new(65);
        let q__1209: Query = from(t___9860.clone()).where_between(t___9861.clone(), SqlPart::new(t___9862.clone()), SqlPart::new(t___9863.clone()));
        let mut t___9868: bool = Some(q__1209.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age BETWEEN 18 AND 65");
        #[derive(Clone)]
        struct ClosureGroup___95 {}
        impl ClosureGroup___95 {
            fn fn__9859(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween ints".to_string());
            }
        }
        let closure_group = ClosureGroup___95 {};
        let fn__9859 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9859())
        };
        test___86.assert(t___9868, fn__9859.clone());
        test___86.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenChained__1819() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___87 = temper_std::testing::Test::new();
        let mut t___9844: SafeIdentifier = sid__546("users");
        let mut t___9845: SqlBuilder = SqlBuilder::new();
        t___9845.append_safe("active = ");
        t___9845.append_boolean(true);
        let mut t___9848: SqlFragment = t___9845.accumulated();
        let q__1211: Query = from(t___9844.clone()).r#where(t___9848.clone()).where_between(sid__546("age"), SqlPart::new(SqlInt32::new(21)), SqlPart::new(SqlInt32::new(30)));
        let mut t___9857: bool = Some(q__1211.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND age BETWEEN 21 AND 30");
        #[derive(Clone)]
        struct ClosureGroup___96 {}
        impl ClosureGroup___96 {
            fn fn__9843(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween chained".to_string());
            }
        }
        let closure_group = ClosureGroup___96 {};
        let fn__9843 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9843())
        };
        test___87.assert(t___9857, fn__9843.clone());
        test___87.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeBasic__1821() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___88 = temper_std::testing::Test::new();
        let mut t___9835: SafeIdentifier = sid__546("users");
        let mut t___9836: SafeIdentifier = sid__546("name");
        let q__1213: Query = from(t___9835.clone()).where_like(t___9836.clone(), "John%");
        let mut t___9841: bool = Some(q__1213.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE 'John%'");
        #[derive(Clone)]
        struct ClosureGroup___97 {}
        impl ClosureGroup___97 {
            fn fn__9834(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike".to_string());
            }
        }
        let closure_group = ClosureGroup___97 {};
        let fn__9834 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9834())
        };
        test___88.assert(t___9841, fn__9834.clone());
        test___88.soft_fail_to_hard()
    }
    #[test]
    fn whereIlikeBasic__1822() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___89 = temper_std::testing::Test::new();
        let mut t___9826: SafeIdentifier = sid__546("users");
        let mut t___9827: SafeIdentifier = sid__546("email");
        let q__1215: Query = from(t___9826.clone()).where_i_like(t___9827.clone(), "%@gmail.com");
        let mut t___9832: bool = Some(q__1215.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email ILIKE '%@gmail.com'");
        #[derive(Clone)]
        struct ClosureGroup___98 {}
        impl ClosureGroup___98 {
            fn fn__9825(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereILike".to_string());
            }
        }
        let closure_group = ClosureGroup___98 {};
        let fn__9825 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9825())
        };
        test___89.assert(t___9832, fn__9825.clone());
        test___89.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWithInjectionAttempt__1823() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___90 = temper_std::testing::Test::new();
        let mut t___9812: SafeIdentifier = sid__546("users");
        let mut t___9813: SafeIdentifier = sid__546("name");
        let q__1217: Query = from(t___9812.clone()).where_like(t___9813.clone(), "'; DROP TABLE users; --");
        let s__1218: std::sync::Arc<String> = q__1217.to_sql().to_string();
        let mut t___9818: bool = temper_core::string::index_of( & s__1218, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___99 {
            s__1218: std::sync::Arc<String>
        }
        impl ClosureGroup___99 {
            fn fn__9811(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like injection escaped: {}", self.s__1218.clone()));
            }
        }
        let closure_group = ClosureGroup___99 {
            s__1218: s__1218.clone()
        };
        let fn__9811 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9811())
        };
        test___90.assert(t___9818, fn__9811.clone());
        let mut t___9822: bool = temper_core::string::index_of( & s__1218, "LIKE", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___100 {
            s__1218: std::sync::Arc<String>
        }
        impl ClosureGroup___100 {
            fn fn__9810(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like structure intact: {}", self.s__1218.clone()));
            }
        }
        let closure_group = ClosureGroup___100 {
            s__1218: s__1218.clone()
        };
        let fn__9810 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9810())
        };
        test___90.assert(t___9822, fn__9810.clone());
        test___90.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWildcardPatterns__1824() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___91 = temper_std::testing::Test::new();
        let mut t___9802: SafeIdentifier = sid__546("users");
        let mut t___9803: SafeIdentifier = sid__546("name");
        let q__1220: Query = from(t___9802.clone()).where_like(t___9803.clone(), "%son%");
        let mut t___9808: bool = Some(q__1220.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE '%son%'");
        #[derive(Clone)]
        struct ClosureGroup___101 {}
        impl ClosureGroup___101 {
            fn fn__9801(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike wildcard".to_string());
            }
        }
        let closure_group = ClosureGroup___101 {};
        let fn__9801 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9801())
        };
        test___91.assert(t___9808, fn__9801.clone());
        test___91.soft_fail_to_hard()
    }
    #[test]
    fn countAllProducesCount__1825() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___92 = temper_std::testing::Test::new();
        let f__1222: SqlFragment = count_all();
        let mut t___9799: bool = Some(f__1222.to_string().as_str()) == Some("COUNT(*)");
        #[derive(Clone)]
        struct ClosureGroup___102 {}
        impl ClosureGroup___102 {
            fn fn__9795(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countAll".to_string());
            }
        }
        let closure_group = ClosureGroup___102 {};
        let fn__9795 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9795())
        };
        test___92.assert(t___9799, fn__9795.clone());
        test___92.soft_fail_to_hard()
    }
    #[test]
    fn countColProducesCountField__1826() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___93 = temper_std::testing::Test::new();
        let f__1224: SqlFragment = count_col(sid__546("id"));
        let mut t___9793: bool = Some(f__1224.to_string().as_str()) == Some("COUNT(id)");
        #[derive(Clone)]
        struct ClosureGroup___103 {}
        impl ClosureGroup___103 {
            fn fn__9788(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countCol".to_string());
            }
        }
        let closure_group = ClosureGroup___103 {};
        let fn__9788 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9788())
        };
        test___93.assert(t___9793, fn__9788.clone());
        test___93.soft_fail_to_hard()
    }
    #[test]
    fn sumColProducesSumField__1827() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___94 = temper_std::testing::Test::new();
        let f__1226: SqlFragment = sum_col(sid__546("amount"));
        let mut t___9786: bool = Some(f__1226.to_string().as_str()) == Some("SUM(amount)");
        #[derive(Clone)]
        struct ClosureGroup___104 {}
        impl ClosureGroup___104 {
            fn fn__9781(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("sumCol".to_string());
            }
        }
        let closure_group = ClosureGroup___104 {};
        let fn__9781 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9781())
        };
        test___94.assert(t___9786, fn__9781.clone());
        test___94.soft_fail_to_hard()
    }
    #[test]
    fn avgColProducesAvgField__1828() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___95 = temper_std::testing::Test::new();
        let f__1228: SqlFragment = avg_col(sid__546("price"));
        let mut t___9779: bool = Some(f__1228.to_string().as_str()) == Some("AVG(price)");
        #[derive(Clone)]
        struct ClosureGroup___105 {}
        impl ClosureGroup___105 {
            fn fn__9774(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("avgCol".to_string());
            }
        }
        let closure_group = ClosureGroup___105 {};
        let fn__9774 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9774())
        };
        test___95.assert(t___9779, fn__9774.clone());
        test___95.soft_fail_to_hard()
    }
    #[test]
    fn minColProducesMinField__1829() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___96 = temper_std::testing::Test::new();
        let f__1230: SqlFragment = min_col(sid__546("created_at"));
        let mut t___9772: bool = Some(f__1230.to_string().as_str()) == Some("MIN(created_at)");
        #[derive(Clone)]
        struct ClosureGroup___106 {}
        impl ClosureGroup___106 {
            fn fn__9767(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("minCol".to_string());
            }
        }
        let closure_group = ClosureGroup___106 {};
        let fn__9767 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9767())
        };
        test___96.assert(t___9772, fn__9767.clone());
        test___96.soft_fail_to_hard()
    }
    #[test]
    fn maxColProducesMaxField__1830() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___97 = temper_std::testing::Test::new();
        let f__1232: SqlFragment = max_col(sid__546("score"));
        let mut t___9765: bool = Some(f__1232.to_string().as_str()) == Some("MAX(score)");
        #[derive(Clone)]
        struct ClosureGroup___107 {}
        impl ClosureGroup___107 {
            fn fn__9760(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("maxCol".to_string());
            }
        }
        let closure_group = ClosureGroup___107 {};
        let fn__9760 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9760())
        };
        test___97.assert(t___9765, fn__9760.clone());
        test___97.soft_fail_to_hard()
    }
    #[test]
    fn selectExprWithAggregate__1831() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___98 = temper_std::testing::Test::new();
        let mut t___9752: SafeIdentifier = sid__546("orders");
        let mut t___9753: SqlFragment = count_all();
        let q__1234: Query = from(t___9752.clone()).select_expr([t___9753.clone()]);
        let mut t___9758: bool = Some(q__1234.to_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM orders");
        #[derive(Clone)]
        struct ClosureGroup___108 {}
        impl ClosureGroup___108 {
            fn fn__9751(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr count".to_string());
            }
        }
        let closure_group = ClosureGroup___108 {};
        let fn__9751 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9751())
        };
        test___98.assert(t___9758, fn__9751.clone());
        test___98.soft_fail_to_hard()
    }
    #[test]
    fn selectExprWithMultipleExpressions__1832() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___99 = temper_std::testing::Test::new();
        let nameFrag__1236: SqlFragment = col(sid__546("users"), sid__546("name"));
        let mut t___9743: SafeIdentifier = sid__546("users");
        let mut t___9744: SqlFragment = count_all();
        let q__1237: Query = from(t___9743.clone()).select_expr([nameFrag__1236.clone(), t___9744.clone()]);
        let mut t___9749: bool = Some(q__1237.to_sql().to_string().as_str()) == Some("SELECT users.name, COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___109 {}
        impl ClosureGroup___109 {
            fn fn__9739(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr multi".to_string());
            }
        }
        let closure_group = ClosureGroup___109 {};
        let fn__9739 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9739())
        };
        test___99.assert(t___9749, fn__9739.clone());
        test___99.soft_fail_to_hard()
    }
    #[test]
    fn selectExprOverridesSelectedFields__1833() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___100 = temper_std::testing::Test::new();
        let mut t___9728: SafeIdentifier = sid__546("users");
        let mut t___9729: SafeIdentifier = sid__546("id");
        let mut t___9730: SafeIdentifier = sid__546("name");
        let q__1239: Query = from(t___9728.clone()).select([t___9729.clone(), t___9730.clone()]).select_expr([count_all()]);
        let mut t___9737: bool = Some(q__1239.to_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___110 {}
        impl ClosureGroup___110 {
            fn fn__9727(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr overrides select".to_string());
            }
        }
        let closure_group = ClosureGroup___110 {};
        let fn__9727 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9727())
        };
        test___100.assert(t___9737, fn__9727.clone());
        test___100.soft_fail_to_hard()
    }
    #[test]
    fn groupBySingleField__1834() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___101 = temper_std::testing::Test::new();
        let mut t___9714: SafeIdentifier = sid__546("orders");
        let mut t___9717: SqlFragment = col(sid__546("orders"), sid__546("status"));
        let mut t___9718: SqlFragment = count_all();
        let q__1241: Query = from(t___9714.clone()).select_expr([t___9717.clone(), t___9718.clone()]).group_by(sid__546("status"));
        let mut t___9725: bool = Some(q__1241.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*) FROM orders GROUP BY status");
        #[derive(Clone)]
        struct ClosureGroup___111 {}
        impl ClosureGroup___111 {
            fn fn__9713(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("groupBy single".to_string());
            }
        }
        let closure_group = ClosureGroup___111 {};
        let fn__9713 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9713())
        };
        test___101.assert(t___9725, fn__9713.clone());
        test___101.soft_fail_to_hard()
    }
    #[test]
    fn groupByMultipleFields__1835() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___102 = temper_std::testing::Test::new();
        let mut t___9703: SafeIdentifier = sid__546("orders");
        let mut t___9704: SafeIdentifier = sid__546("status");
        let q__1243: Query = from(t___9703.clone()).group_by(t___9704.clone()).group_by(sid__546("category"));
        let mut t___9711: bool = Some(q__1243.to_sql().to_string().as_str()) == Some("SELECT * FROM orders GROUP BY status, category");
        #[derive(Clone)]
        struct ClosureGroup___112 {}
        impl ClosureGroup___112 {
            fn fn__9702(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("groupBy multiple".to_string());
            }
        }
        let closure_group = ClosureGroup___112 {};
        let fn__9702 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9702())
        };
        test___102.assert(t___9711, fn__9702.clone());
        test___102.soft_fail_to_hard()
    }
    #[test]
    fn havingBasic__1836() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___103 = temper_std::testing::Test::new();
        let mut t___9684: SafeIdentifier = sid__546("orders");
        let mut t___9687: SqlFragment = col(sid__546("orders"), sid__546("status"));
        let mut t___9688: SqlFragment = count_all();
        let mut t___9691: Query = from(t___9684.clone()).select_expr([t___9687.clone(), t___9688.clone()]).group_by(sid__546("status"));
        let mut t___9692: SqlBuilder = SqlBuilder::new();
        t___9692.append_safe("COUNT(*) > ");
        t___9692.append_int32(5);
        let q__1245: Query = t___9691.having(t___9692.accumulated());
        let mut t___9700: bool = Some(q__1245.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*) FROM orders GROUP BY status HAVING COUNT(*) > 5");
        #[derive(Clone)]
        struct ClosureGroup___113 {}
        impl ClosureGroup___113 {
            fn fn__9683(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("having basic".to_string());
            }
        }
        let closure_group = ClosureGroup___113 {};
        let fn__9683 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9683())
        };
        test___103.assert(t___9700, fn__9683.clone());
        test___103.soft_fail_to_hard()
    }
    #[test]
    fn orHaving__1838() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___104 = temper_std::testing::Test::new();
        let mut t___9665: SafeIdentifier = sid__546("orders");
        let mut t___9666: SafeIdentifier = sid__546("status");
        let mut t___9667: Query = from(t___9665.clone()).group_by(t___9666.clone());
        let mut t___9668: SqlBuilder = SqlBuilder::new();
        t___9668.append_safe("COUNT(*) > ");
        t___9668.append_int32(5);
        let mut t___9672: Query = t___9667.having(t___9668.accumulated());
        let mut t___9673: SqlBuilder = SqlBuilder::new();
        t___9673.append_safe("SUM(total) > ");
        t___9673.append_int32(1000);
        let q__1247: Query = t___9672.or_having(t___9673.accumulated());
        let mut t___9681: bool = Some(q__1247.to_sql().to_string().as_str()) == Some("SELECT * FROM orders GROUP BY status HAVING COUNT(*) > 5 OR SUM(total) > 1000");
        #[derive(Clone)]
        struct ClosureGroup___114 {}
        impl ClosureGroup___114 {
            fn fn__9664(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orHaving".to_string());
            }
        }
        let closure_group = ClosureGroup___114 {};
        let fn__9664 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9664())
        };
        test___104.assert(t___9681, fn__9664.clone());
        test___104.soft_fail_to_hard()
    }
    #[test]
    fn distinctBasic__1841() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___105 = temper_std::testing::Test::new();
        let mut t___9655: SafeIdentifier = sid__546("users");
        let mut t___9656: SafeIdentifier = sid__546("name");
        let q__1249: Query = from(t___9655.clone()).select([t___9656.clone()]).distinct();
        let mut t___9662: bool = Some(q__1249.to_sql().to_string().as_str()) == Some("SELECT DISTINCT name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___115 {}
        impl ClosureGroup___115 {
            fn fn__9654(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("distinct".to_string());
            }
        }
        let closure_group = ClosureGroup___115 {};
        let fn__9654 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9654())
        };
        test___105.assert(t___9662, fn__9654.clone());
        test___105.soft_fail_to_hard()
    }
    #[test]
    fn distinctWithWhere__1842() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___106 = temper_std::testing::Test::new();
        let mut t___9640: SafeIdentifier = sid__546("users");
        let mut t___9641: SafeIdentifier = sid__546("email");
        let mut t___9642: Query = from(t___9640.clone()).select([t___9641.clone()]);
        let mut t___9643: SqlBuilder = SqlBuilder::new();
        t___9643.append_safe("active = ");
        t___9643.append_boolean(true);
        let q__1251: Query = t___9642.r#where(t___9643.accumulated()).distinct();
        let mut t___9652: bool = Some(q__1251.to_sql().to_string().as_str()) == Some("SELECT DISTINCT email FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___116 {}
        impl ClosureGroup___116 {
            fn fn__9639(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("distinct with where".to_string());
            }
        }
        let closure_group = ClosureGroup___116 {};
        let fn__9639 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9639())
        };
        test___106.assert(t___9652, fn__9639.clone());
        test___106.soft_fail_to_hard()
    }
    #[test]
    fn countSqlBare__1844() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___107 = temper_std::testing::Test::new();
        let q__1253: Query = from(sid__546("users"));
        let mut t___9637: bool = Some(q__1253.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___117 {}
        impl ClosureGroup___117 {
            fn fn__9632(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql bare".to_string());
            }
        }
        let closure_group = ClosureGroup___117 {};
        let fn__9632 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9632())
        };
        test___107.assert(t___9637, fn__9632.clone());
        test___107.soft_fail_to_hard()
    }
    #[test]
    fn countSqlWithWhere__1845() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___108 = temper_std::testing::Test::new();
        let mut t___9621: SafeIdentifier = sid__546("users");
        let mut t___9622: SqlBuilder = SqlBuilder::new();
        t___9622.append_safe("active = ");
        t___9622.append_boolean(true);
        let mut t___9625: SqlFragment = t___9622.accumulated();
        let q__1255: Query = from(t___9621.clone()).r#where(t___9625.clone());
        let mut t___9630: bool = Some(q__1255.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___118 {}
        impl ClosureGroup___118 {
            fn fn__9620(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql with where".to_string());
            }
        }
        let closure_group = ClosureGroup___118 {};
        let fn__9620 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9620())
        };
        test___108.assert(t___9630, fn__9620.clone());
        test___108.soft_fail_to_hard()
    }
    #[test]
    fn countSqlWithJoin__1847() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___109 = temper_std::testing::Test::new();
        let mut t___9604: SafeIdentifier = sid__546("users");
        let mut t___9605: SafeIdentifier = sid__546("orders");
        let mut t___9606: SqlBuilder = SqlBuilder::new();
        t___9606.append_safe("users.id = orders.user_id");
        let mut t___9608: SqlFragment = t___9606.accumulated();
        let mut t___9609: Query = from(t___9604.clone()).inner_join(t___9605.clone(), t___9608.clone());
        let mut t___9610: SqlBuilder = SqlBuilder::new();
        t___9610.append_safe("orders.total > ");
        t___9610.append_int32(100);
        let q__1257: Query = t___9609.r#where(t___9610.accumulated());
        let mut t___9618: bool = Some(q__1257.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100");
        #[derive(Clone)]
        struct ClosureGroup___119 {}
        impl ClosureGroup___119 {
            fn fn__9603(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql with join".to_string());
            }
        }
        let closure_group = ClosureGroup___119 {};
        let fn__9603 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9603())
        };
        test___109.assert(t___9618, fn__9603.clone());
        test___109.soft_fail_to_hard()
    }
    #[test]
    fn countSqlDropsOrderByLimitOffset__1850() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___110 = temper_std::testing::Test::new();
        let mut t___9590: SafeIdentifier;
        let mut t___9591: SqlBuilder;
        let mut t___9594: SqlFragment;
        let mut t___4906: Query;
        let mut t___4907: Query;
        let q__1259: Query;
        'ok___11516: {
            'orelse___2031: {
                t___9590 = sid__546("users");
                t___9591 = SqlBuilder::new();
                t___9591.append_safe("active = ");
                t___9591.append_boolean(true);
                t___9594 = t___9591.accumulated();
                t___4906 = match from(t___9590.clone()).r#where(t___9594.clone()).order_by(sid__546("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___2031
                };
                t___4907 = match t___4906.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___2031
                };
                q__1259 = t___4907.clone();
                break 'ok___11516;
            }
            q__1259 = panic!();
        }
        let s__1260: std::sync::Arc<String> = q__1259.count_sql().to_string();
        let mut t___9601: bool = Some(s__1260.as_str()) == Some("SELECT COUNT(*) FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___120 {
            s__1260: std::sync::Arc<String>
        }
        impl ClosureGroup___120 {
            fn fn__9589(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("countSql drops extras: {}", self.s__1260.clone()));
            }
        }
        let closure_group = ClosureGroup___120 {
            s__1260: s__1260.clone()
        };
        let fn__9589 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9589())
        };
        test___110.assert(t___9601, fn__9589.clone());
        test___110.soft_fail_to_hard()
    }
    #[test]
    fn fullAggregationQuery__1852() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___111 = temper_std::testing::Test::new();
        let mut t___9557: SafeIdentifier = sid__546("orders");
        let mut t___9560: SqlFragment = col(sid__546("orders"), sid__546("status"));
        let mut t___9561: SqlFragment = count_all();
        let mut t___9563: SqlFragment = sum_col(sid__546("total"));
        let mut t___9564: Query = from(t___9557.clone()).select_expr([t___9560.clone(), t___9561.clone(), t___9563.clone()]);
        let mut t___9565: SafeIdentifier = sid__546("users");
        let mut t___9566: SqlBuilder = SqlBuilder::new();
        t___9566.append_safe("orders.user_id = users.id");
        let mut t___9569: Query = t___9564.inner_join(t___9565.clone(), t___9566.accumulated());
        let mut t___9570: SqlBuilder = SqlBuilder::new();
        t___9570.append_safe("users.active = ");
        t___9570.append_boolean(true);
        let mut t___9576: Query = t___9569.r#where(t___9570.accumulated()).group_by(sid__546("status"));
        let mut t___9577: SqlBuilder = SqlBuilder::new();
        t___9577.append_safe("COUNT(*) > ");
        t___9577.append_int32(3);
        let q__1262: Query = t___9576.having(t___9577.accumulated()).order_by(sid__546("status"), true);
        let expected__1263: std::sync::Arc<String> = std::sync::Arc::new("SELECT orders.status, COUNT(*), SUM(total) FROM orders INNER JOIN users ON orders.user_id = users.id WHERE users.active = TRUE GROUP BY status HAVING COUNT(*) > 3 ORDER BY status ASC".to_string());
        let mut t___9587: bool = Some(q__1262.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*), SUM(total) FROM orders INNER JOIN users ON orders.user_id = users.id WHERE users.active = TRUE GROUP BY status HAVING COUNT(*) > 3 ORDER BY status ASC");
        #[derive(Clone)]
        struct ClosureGroup___121 {}
        impl ClosureGroup___121 {
            fn fn__9556(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full aggregation".to_string());
            }
        }
        let closure_group = ClosureGroup___121 {};
        let fn__9556 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9556())
        };
        test___111.assert(t___9587, fn__9556.clone());
        test___111.soft_fail_to_hard()
    }
    #[test]
    fn unionSql__1856() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___112 = temper_std::testing::Test::new();
        let mut t___9539: SafeIdentifier = sid__546("users");
        let mut t___9540: SqlBuilder = SqlBuilder::new();
        t___9540.append_safe("role = ");
        t___9540.append_string("admin");
        let mut t___9543: SqlFragment = t___9540.accumulated();
        let a__1265: Query = from(t___9539.clone()).r#where(t___9543.clone());
        let mut t___9545: SafeIdentifier = sid__546("users");
        let mut t___9546: SqlBuilder = SqlBuilder::new();
        t___9546.append_safe("role = ");
        t___9546.append_string("moderator");
        let mut t___9549: SqlFragment = t___9546.accumulated();
        let b__1266: Query = from(t___9545.clone()).r#where(t___9549.clone());
        let s__1267: std::sync::Arc<String> = union_sql(a__1265.clone(), b__1266.clone()).to_string();
        let mut t___9554: bool = Some(s__1267.as_str()) == Some("(SELECT * FROM users WHERE role = 'admin') UNION (SELECT * FROM users WHERE role = 'moderator')");
        #[derive(Clone)]
        struct ClosureGroup___122 {
            s__1267: std::sync::Arc<String>
        }
        impl ClosureGroup___122 {
            fn fn__9538(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("unionSql: {}", self.s__1267.clone()));
            }
        }
        let closure_group = ClosureGroup___122 {
            s__1267: s__1267.clone()
        };
        let fn__9538 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9538())
        };
        test___112.assert(t___9554, fn__9538.clone());
        test___112.soft_fail_to_hard()
    }
    #[test]
    fn unionAllSql__1859() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___113 = temper_std::testing::Test::new();
        let mut t___9527: SafeIdentifier = sid__546("users");
        let mut t___9528: SafeIdentifier = sid__546("name");
        let a__1269: Query = from(t___9527.clone()).select([t___9528.clone()]);
        let mut t___9530: SafeIdentifier = sid__546("contacts");
        let mut t___9531: SafeIdentifier = sid__546("name");
        let b__1270: Query = from(t___9530.clone()).select([t___9531.clone()]);
        let s__1271: std::sync::Arc<String> = union_all_sql(a__1269.clone(), b__1270.clone()).to_string();
        let mut t___9536: bool = Some(s__1271.as_str()) == Some("(SELECT name FROM users) UNION ALL (SELECT name FROM contacts)");
        #[derive(Clone)]
        struct ClosureGroup___123 {
            s__1271: std::sync::Arc<String>
        }
        impl ClosureGroup___123 {
            fn fn__9526(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("unionAllSql: {}", self.s__1271.clone()));
            }
        }
        let closure_group = ClosureGroup___123 {
            s__1271: s__1271.clone()
        };
        let fn__9526 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9526())
        };
        test___113.assert(t___9536, fn__9526.clone());
        test___113.soft_fail_to_hard()
    }
    #[test]
    fn intersectSql__1860() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___114 = temper_std::testing::Test::new();
        let mut t___9515: SafeIdentifier = sid__546("users");
        let mut t___9516: SafeIdentifier = sid__546("email");
        let a__1273: Query = from(t___9515.clone()).select([t___9516.clone()]);
        let mut t___9518: SafeIdentifier = sid__546("subscribers");
        let mut t___9519: SafeIdentifier = sid__546("email");
        let b__1274: Query = from(t___9518.clone()).select([t___9519.clone()]);
        let s__1275: std::sync::Arc<String> = intersect_sql(a__1273.clone(), b__1274.clone()).to_string();
        let mut t___9524: bool = Some(s__1275.as_str()) == Some("(SELECT email FROM users) INTERSECT (SELECT email FROM subscribers)");
        #[derive(Clone)]
        struct ClosureGroup___124 {
            s__1275: std::sync::Arc<String>
        }
        impl ClosureGroup___124 {
            fn fn__9514(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("intersectSql: {}", self.s__1275.clone()));
            }
        }
        let closure_group = ClosureGroup___124 {
            s__1275: s__1275.clone()
        };
        let fn__9514 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9514())
        };
        test___114.assert(t___9524, fn__9514.clone());
        test___114.soft_fail_to_hard()
    }
    #[test]
    fn exceptSql__1861() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___115 = temper_std::testing::Test::new();
        let mut t___9503: SafeIdentifier = sid__546("users");
        let mut t___9504: SafeIdentifier = sid__546("id");
        let a__1277: Query = from(t___9503.clone()).select([t___9504.clone()]);
        let mut t___9506: SafeIdentifier = sid__546("banned");
        let mut t___9507: SafeIdentifier = sid__546("id");
        let b__1278: Query = from(t___9506.clone()).select([t___9507.clone()]);
        let s__1279: std::sync::Arc<String> = except_sql(a__1277.clone(), b__1278.clone()).to_string();
        let mut t___9512: bool = Some(s__1279.as_str()) == Some("(SELECT id FROM users) EXCEPT (SELECT id FROM banned)");
        #[derive(Clone)]
        struct ClosureGroup___125 {
            s__1279: std::sync::Arc<String>
        }
        impl ClosureGroup___125 {
            fn fn__9502(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("exceptSql: {}", self.s__1279.clone()));
            }
        }
        let closure_group = ClosureGroup___125 {
            s__1279: s__1279.clone()
        };
        let fn__9502 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9502())
        };
        test___115.assert(t___9512, fn__9502.clone());
        test___115.soft_fail_to_hard()
    }
    #[test]
    fn subqueryWithAlias__1862() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___116 = temper_std::testing::Test::new();
        let mut t___9488: SafeIdentifier = sid__546("orders");
        let mut t___9489: SafeIdentifier = sid__546("user_id");
        let mut t___9490: Query = from(t___9488.clone()).select([t___9489.clone()]);
        let mut t___9491: SqlBuilder = SqlBuilder::new();
        t___9491.append_safe("total > ");
        t___9491.append_int32(100);
        let inner__1281: Query = t___9490.r#where(t___9491.accumulated());
        let s__1282: std::sync::Arc<String> = subquery(inner__1281.clone(), sid__546("big_orders")).to_string();
        let mut t___9500: bool = Some(s__1282.as_str()) == Some("(SELECT user_id FROM orders WHERE total > 100) AS big_orders");
        #[derive(Clone)]
        struct ClosureGroup___126 {
            s__1282: std::sync::Arc<String>
        }
        impl ClosureGroup___126 {
            fn fn__9487(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("subquery: {}", self.s__1282.clone()));
            }
        }
        let closure_group = ClosureGroup___126 {
            s__1282: s__1282.clone()
        };
        let fn__9487 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9487())
        };
        test___116.assert(t___9500, fn__9487.clone());
        test___116.soft_fail_to_hard()
    }
    #[test]
    fn existsSql__1864() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___117 = temper_std::testing::Test::new();
        let mut t___9477: SafeIdentifier = sid__546("orders");
        let mut t___9478: SqlBuilder = SqlBuilder::new();
        t___9478.append_safe("orders.user_id = users.id");
        let mut t___9480: SqlFragment = t___9478.accumulated();
        let inner__1284: Query = from(t___9477.clone()).r#where(t___9480.clone());
        let s__1285: std::sync::Arc<String> = exists_sql(inner__1284.clone()).to_string();
        let mut t___9485: bool = Some(s__1285.as_str()) == Some("EXISTS (SELECT * FROM orders WHERE orders.user_id = users.id)");
        #[derive(Clone)]
        struct ClosureGroup___127 {
            s__1285: std::sync::Arc<String>
        }
        impl ClosureGroup___127 {
            fn fn__9476(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("existsSql: {}", self.s__1285.clone()));
            }
        }
        let closure_group = ClosureGroup___127 {
            s__1285: s__1285.clone()
        };
        let fn__9476 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9476())
        };
        test___117.assert(t___9485, fn__9476.clone());
        test___117.soft_fail_to_hard()
    }
    #[test]
    fn whereInSubquery__1866() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___118 = temper_std::testing::Test::new();
        let mut t___9460: SafeIdentifier = sid__546("orders");
        let mut t___9461: SafeIdentifier = sid__546("user_id");
        let mut t___9462: Query = from(t___9460.clone()).select([t___9461.clone()]);
        let mut t___9463: SqlBuilder = SqlBuilder::new();
        t___9463.append_safe("total > ");
        t___9463.append_int32(1000);
        let sub__1287: Query = t___9462.r#where(t___9463.accumulated());
        let mut t___9468: SafeIdentifier = sid__546("users");
        let mut t___9469: SafeIdentifier = sid__546("id");
        let q__1288: Query = from(t___9468.clone()).where_in_subquery(t___9469.clone(), sub__1287.clone());
        let s__1289: std::sync::Arc<String> = q__1288.to_sql().to_string();
        let mut t___9474: bool = Some(s__1289.as_str()) == Some("SELECT * FROM users WHERE id IN (SELECT user_id FROM orders WHERE total > 1000)");
        #[derive(Clone)]
        struct ClosureGroup___128 {
            s__1289: std::sync::Arc<String>
        }
        impl ClosureGroup___128 {
            fn fn__9459(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("whereInSubquery: {}", self.s__1289.clone()));
            }
        }
        let closure_group = ClosureGroup___128 {
            s__1289: s__1289.clone()
        };
        let fn__9459 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9459())
        };
        test___118.assert(t___9474, fn__9459.clone());
        test___118.soft_fail_to_hard()
    }
    #[test]
    fn setOperationWithWhereOnEachSide__1868() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___119 = temper_std::testing::Test::new();
        let mut t___9437: SafeIdentifier = sid__546("users");
        let mut t___9438: SqlBuilder = SqlBuilder::new();
        t___9438.append_safe("age > ");
        t___9438.append_int32(18);
        let mut t___9441: SqlFragment = t___9438.accumulated();
        let mut t___9442: Query = from(t___9437.clone()).r#where(t___9441.clone());
        let mut t___9443: SqlBuilder = SqlBuilder::new();
        t___9443.append_safe("active = ");
        t___9443.append_boolean(true);
        let a__1291: Query = t___9442.r#where(t___9443.accumulated());
        let mut t___9448: SafeIdentifier = sid__546("users");
        let mut t___9449: SqlBuilder = SqlBuilder::new();
        t___9449.append_safe("role = ");
        t___9449.append_string("vip");
        let mut t___9452: SqlFragment = t___9449.accumulated();
        let b__1292: Query = from(t___9448.clone()).r#where(t___9452.clone());
        let s__1293: std::sync::Arc<String> = union_sql(a__1291.clone(), b__1292.clone()).to_string();
        let mut t___9457: bool = Some(s__1293.as_str()) == Some("(SELECT * FROM users WHERE age > 18 AND active = TRUE) UNION (SELECT * FROM users WHERE role = 'vip')");
        #[derive(Clone)]
        struct ClosureGroup___129 {
            s__1293: std::sync::Arc<String>
        }
        impl ClosureGroup___129 {
            fn fn__9436(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("union with where: {}", self.s__1293.clone()));
            }
        }
        let closure_group = ClosureGroup___129 {
            s__1293: s__1293.clone()
        };
        let fn__9436 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9436())
        };
        test___119.assert(t___9457, fn__9436.clone());
        test___119.soft_fail_to_hard()
    }
    #[test]
    fn whereInSubqueryChainedWithWhere__1872() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___120 = temper_std::testing::Test::new();
        let mut t___9420: SafeIdentifier = sid__546("orders");
        let mut t___9421: SafeIdentifier = sid__546("user_id");
        let sub__1295: Query = from(t___9420.clone()).select([t___9421.clone()]);
        let mut t___9423: SafeIdentifier = sid__546("users");
        let mut t___9424: SqlBuilder = SqlBuilder::new();
        t___9424.append_safe("active = ");
        t___9424.append_boolean(true);
        let mut t___9427: SqlFragment = t___9424.accumulated();
        let q__1296: Query = from(t___9423.clone()).r#where(t___9427.clone()).where_in_subquery(sid__546("id"), sub__1295.clone());
        let s__1297: std::sync::Arc<String> = q__1296.to_sql().to_string();
        let mut t___9434: bool = Some(s__1297.as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND id IN (SELECT user_id FROM orders)");
        #[derive(Clone)]
        struct ClosureGroup___130 {
            s__1297: std::sync::Arc<String>
        }
        impl ClosureGroup___130 {
            fn fn__9419(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("whereInSubquery chained: {}", self.s__1297.clone()));
            }
        }
        let closure_group = ClosureGroup___130 {
            s__1297: s__1297.clone()
        };
        let fn__9419 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9419())
        };
        test___120.assert(t___9434, fn__9419.clone());
        test___120.soft_fail_to_hard()
    }
    #[test]
    fn existsSqlUsedInWhere__1874() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___121 = temper_std::testing::Test::new();
        let mut t___9406: SafeIdentifier = sid__546("orders");
        let mut t___9407: SqlBuilder = SqlBuilder::new();
        t___9407.append_safe("orders.user_id = users.id");
        let mut t___9409: SqlFragment = t___9407.accumulated();
        let sub__1299: Query = from(t___9406.clone()).r#where(t___9409.clone());
        let mut t___9411: SafeIdentifier = sid__546("users");
        let mut t___9412: SqlFragment = exists_sql(sub__1299.clone());
        let q__1300: Query = from(t___9411.clone()).r#where(t___9412.clone());
        let s__1301: std::sync::Arc<String> = q__1300.to_sql().to_string();
        let mut t___9417: bool = Some(s__1301.as_str()) == Some("SELECT * FROM users WHERE EXISTS (SELECT * FROM orders WHERE orders.user_id = users.id)");
        #[derive(Clone)]
        struct ClosureGroup___131 {
            s__1301: std::sync::Arc<String>
        }
        impl ClosureGroup___131 {
            fn fn__9405(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("exists in where: {}", self.s__1301.clone()));
            }
        }
        let closure_group = ClosureGroup___131 {
            s__1301: s__1301.clone()
        };
        let fn__9405 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9405())
        };
        test___121.assert(t___9417, fn__9405.clone());
        test___121.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBasic__1876() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___122 = temper_std::testing::Test::new();
        let mut t___9392: SafeIdentifier;
        let mut t___9393: SafeIdentifier;
        let mut t___9394: SqlString;
        let mut t___9395: UpdateQuery;
        let mut t___9396: SqlBuilder;
        let mut t___4728: SqlFragment;
        let q__1303: SqlFragment;
        'ok___11517: {
            'orelse___2032: {
                t___9392 = sid__546("users");
                t___9393 = sid__546("name");
                t___9394 = SqlString::new("Alice");
                t___9395 = update(t___9392.clone()).set(t___9393.clone(), SqlPart::new(t___9394.clone()));
                t___9396 = SqlBuilder::new();
                t___9396.append_safe("id = ");
                t___9396.append_int32(1);
                t___4728 = match t___9395.r#where(t___9396.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2032
                };
                q__1303 = t___4728.clone();
                break 'ok___11517;
            }
            q__1303 = panic!();
        }
        let mut t___9403: bool = Some(q__1303.to_string().as_str()) == Some("UPDATE users SET name = 'Alice' WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___132 {}
        impl ClosureGroup___132 {
            fn fn__9391(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update basic".to_string());
            }
        }
        let closure_group = ClosureGroup___132 {};
        let fn__9391 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9391())
        };
        test___122.assert(t___9403, fn__9391.clone());
        test___122.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryMultipleSet__1878() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___123 = temper_std::testing::Test::new();
        let mut t___9375: SafeIdentifier;
        let mut t___9376: SafeIdentifier;
        let mut t___9377: SqlString;
        let mut t___9381: UpdateQuery;
        let mut t___9382: SqlBuilder;
        let mut t___4713: SqlFragment;
        let q__1305: SqlFragment;
        'ok___11518: {
            'orelse___2033: {
                t___9375 = sid__546("users");
                t___9376 = sid__546("name");
                t___9377 = SqlString::new("Bob");
                t___9381 = update(t___9375.clone()).set(t___9376.clone(), SqlPart::new(t___9377.clone())).set(sid__546("age"), SqlPart::new(SqlInt32::new(30)));
                t___9382 = SqlBuilder::new();
                t___9382.append_safe("id = ");
                t___9382.append_int32(2);
                t___4713 = match t___9381.r#where(t___9382.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2033
                };
                q__1305 = t___4713.clone();
                break 'ok___11518;
            }
            q__1305 = panic!();
        }
        let mut t___9389: bool = Some(q__1305.to_string().as_str()) == Some("UPDATE users SET name = 'Bob', age = 30 WHERE id = 2");
        #[derive(Clone)]
        struct ClosureGroup___133 {}
        impl ClosureGroup___133 {
            fn fn__9374(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update multi set".to_string());
            }
        }
        let closure_group = ClosureGroup___133 {};
        let fn__9374 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9374())
        };
        test___123.assert(t___9389, fn__9374.clone());
        test___123.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryMultipleWhere__1880() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___124 = temper_std::testing::Test::new();
        let mut t___9356: SafeIdentifier;
        let mut t___9357: SafeIdentifier;
        let mut t___9358: SqlBoolean;
        let mut t___9359: UpdateQuery;
        let mut t___9360: SqlBuilder;
        let mut t___9364: UpdateQuery;
        let mut t___9365: SqlBuilder;
        let mut t___4695: SqlFragment;
        let q__1307: SqlFragment;
        'ok___11519: {
            'orelse___2034: {
                t___9356 = sid__546("users");
                t___9357 = sid__546("active");
                t___9358 = SqlBoolean::new(false);
                t___9359 = update(t___9356.clone()).set(t___9357.clone(), SqlPart::new(t___9358.clone()));
                t___9360 = SqlBuilder::new();
                t___9360.append_safe("age < ");
                t___9360.append_int32(18);
                t___9364 = t___9359.r#where(t___9360.accumulated());
                t___9365 = SqlBuilder::new();
                t___9365.append_safe("role = ");
                t___9365.append_string("guest");
                t___4695 = match t___9364.r#where(t___9365.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2034
                };
                q__1307 = t___4695.clone();
                break 'ok___11519;
            }
            q__1307 = panic!();
        }
        let mut t___9372: bool = Some(q__1307.to_string().as_str()) == Some("UPDATE users SET active = FALSE WHERE age < 18 AND role = 'guest'");
        #[derive(Clone)]
        struct ClosureGroup___134 {}
        impl ClosureGroup___134 {
            fn fn__9355(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update multi where".to_string());
            }
        }
        let closure_group = ClosureGroup___134 {};
        let fn__9355 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9355())
        };
        test___124.assert(t___9372, fn__9355.clone());
        test___124.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryOrWhere__1883() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___125 = temper_std::testing::Test::new();
        let mut t___9337: SafeIdentifier;
        let mut t___9338: SafeIdentifier;
        let mut t___9339: SqlString;
        let mut t___9340: UpdateQuery;
        let mut t___9341: SqlBuilder;
        let mut t___9345: UpdateQuery;
        let mut t___9346: SqlBuilder;
        let mut t___4674: SqlFragment;
        let q__1309: SqlFragment;
        'ok___11520: {
            'orelse___2035: {
                t___9337 = sid__546("users");
                t___9338 = sid__546("status");
                t___9339 = SqlString::new("banned");
                t___9340 = update(t___9337.clone()).set(t___9338.clone(), SqlPart::new(t___9339.clone()));
                t___9341 = SqlBuilder::new();
                t___9341.append_safe("spam_count > ");
                t___9341.append_int32(10);
                t___9345 = t___9340.r#where(t___9341.accumulated());
                t___9346 = SqlBuilder::new();
                t___9346.append_safe("reported = ");
                t___9346.append_boolean(true);
                t___4674 = match t___9345.or_where(t___9346.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2035
                };
                q__1309 = t___4674.clone();
                break 'ok___11520;
            }
            q__1309 = panic!();
        }
        let mut t___9353: bool = Some(q__1309.to_string().as_str()) == Some("UPDATE users SET status = 'banned' WHERE spam_count > 10 OR reported = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___135 {}
        impl ClosureGroup___135 {
            fn fn__9336(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___135 {};
        let fn__9336 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9336())
        };
        test___125.assert(t___9353, fn__9336.clone());
        test___125.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBubblesWithoutWhere__1886() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___126 = temper_std::testing::Test::new();
        let mut t___9330: SafeIdentifier;
        let mut t___9331: SafeIdentifier;
        let mut t___9332: SqlInt32;
        let didBubble__1311: bool;
        'ok___11521: {
            'orelse___2036: {
                t___9330 = sid__546("users");
                t___9331 = sid__546("x");
                t___9332 = SqlInt32::new(1);
                match update(t___9330.clone()).set(t___9331.clone(), SqlPart::new(t___9332.clone())).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2036
                };
                didBubble__1311 = false;
                break 'ok___11521;
            }
            didBubble__1311 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___136 {}
        impl ClosureGroup___136 {
            fn fn__9329(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update without WHERE should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___136 {};
        let fn__9329 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9329())
        };
        test___126.assert(didBubble__1311, fn__9329.clone());
        test___126.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBubblesWithoutSet__1887() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___127 = temper_std::testing::Test::new();
        let mut t___9321: SafeIdentifier;
        let mut t___9322: SqlBuilder;
        let mut t___9325: SqlFragment;
        let didBubble__1313: bool;
        'ok___11522: {
            'orelse___2037: {
                t___9321 = sid__546("users");
                t___9322 = SqlBuilder::new();
                t___9322.append_safe("id = ");
                t___9322.append_int32(1);
                t___9325 = t___9322.accumulated();
                match update(t___9321.clone()).r#where(t___9325.clone()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2037
                };
                didBubble__1313 = false;
                break 'ok___11522;
            }
            didBubble__1313 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___137 {}
        impl ClosureGroup___137 {
            fn fn__9320(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update without SET should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___137 {};
        let fn__9320 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9320())
        };
        test___127.assert(didBubble__1313, fn__9320.clone());
        test___127.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryWithLimit__1889() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___128 = temper_std::testing::Test::new();
        let mut t___9307: SafeIdentifier;
        let mut t___9308: SafeIdentifier;
        let mut t___9309: SqlBoolean;
        let mut t___9310: UpdateQuery;
        let mut t___9311: SqlBuilder;
        let mut t___4637: UpdateQuery;
        let mut t___4638: SqlFragment;
        let q__1315: SqlFragment;
        'ok___11523: {
            'orelse___2038: {
                t___9307 = sid__546("users");
                t___9308 = sid__546("active");
                t___9309 = SqlBoolean::new(false);
                t___9310 = update(t___9307.clone()).set(t___9308.clone(), SqlPart::new(t___9309.clone()));
                t___9311 = SqlBuilder::new();
                t___9311.append_safe("last_login < ");
                t___9311.append_string("2024-01-01");
                t___4637 = match t___9310.r#where(t___9311.accumulated()).limit(100) {
                    Ok(x) => x,
                    _ => break 'orelse___2038
                };
                t___4638 = match t___4637.to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2038
                };
                q__1315 = t___4638.clone();
                break 'ok___11523;
            }
            q__1315 = panic!();
        }
        let mut t___9318: bool = Some(q__1315.to_string().as_str()) == Some("UPDATE users SET active = FALSE WHERE last_login < '2024-01-01' LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___138 {}
        impl ClosureGroup___138 {
            fn fn__9306(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update limit".to_string());
            }
        }
        let closure_group = ClosureGroup___138 {};
        let fn__9306 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9306())
        };
        test___128.assert(t___9318, fn__9306.clone());
        test___128.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryEscaping__1891() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___129 = temper_std::testing::Test::new();
        let mut t___9293: SafeIdentifier;
        let mut t___9294: SafeIdentifier;
        let mut t___9295: SqlString;
        let mut t___9296: UpdateQuery;
        let mut t___9297: SqlBuilder;
        let mut t___4622: SqlFragment;
        let q__1317: SqlFragment;
        'ok___11524: {
            'orelse___2039: {
                t___9293 = sid__546("users");
                t___9294 = sid__546("bio");
                t___9295 = SqlString::new("It's a test");
                t___9296 = update(t___9293.clone()).set(t___9294.clone(), SqlPart::new(t___9295.clone()));
                t___9297 = SqlBuilder::new();
                t___9297.append_safe("id = ");
                t___9297.append_int32(1);
                t___4622 = match t___9296.r#where(t___9297.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2039
                };
                q__1317 = t___4622.clone();
                break 'ok___11524;
            }
            q__1317 = panic!();
        }
        let mut t___9304: bool = Some(q__1317.to_string().as_str()) == Some("UPDATE users SET bio = 'It''s a test' WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___139 {}
        impl ClosureGroup___139 {
            fn fn__9292(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update escaping".to_string());
            }
        }
        let closure_group = ClosureGroup___139 {};
        let fn__9292 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9292())
        };
        test___129.assert(t___9304, fn__9292.clone());
        test___129.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryBasic__1893() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___130 = temper_std::testing::Test::new();
        let mut t___9282: SafeIdentifier;
        let mut t___9283: SqlBuilder;
        let mut t___9286: SqlFragment;
        let mut t___4607: SqlFragment;
        let q__1319: SqlFragment;
        'ok___11525: {
            'orelse___2040: {
                t___9282 = sid__546("users");
                t___9283 = SqlBuilder::new();
                t___9283.append_safe("id = ");
                t___9283.append_int32(1);
                t___9286 = t___9283.accumulated();
                t___4607 = match delete_from(t___9282.clone()).r#where(t___9286.clone()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2040
                };
                q__1319 = t___4607.clone();
                break 'ok___11525;
            }
            q__1319 = panic!();
        }
        let mut t___9290: bool = Some(q__1319.to_string().as_str()) == Some("DELETE FROM users WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___140 {}
        impl ClosureGroup___140 {
            fn fn__9281(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete basic".to_string());
            }
        }
        let closure_group = ClosureGroup___140 {};
        let fn__9281 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9281())
        };
        test___130.assert(t___9290, fn__9281.clone());
        test___130.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryMultipleWhere__1895() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___131 = temper_std::testing::Test::new();
        let mut t___9266: SafeIdentifier;
        let mut t___9267: SqlBuilder;
        let mut t___9270: SqlFragment;
        let mut t___9271: DeleteQuery;
        let mut t___9272: SqlBuilder;
        let mut t___4595: SqlFragment;
        let q__1321: SqlFragment;
        'ok___11526: {
            'orelse___2041: {
                t___9266 = sid__546("logs");
                t___9267 = SqlBuilder::new();
                t___9267.append_safe("created_at < ");
                t___9267.append_string("2024-01-01");
                t___9270 = t___9267.accumulated();
                t___9271 = delete_from(t___9266.clone()).r#where(t___9270.clone());
                t___9272 = SqlBuilder::new();
                t___9272.append_safe("level = ");
                t___9272.append_string("debug");
                t___4595 = match t___9271.r#where(t___9272.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2041
                };
                q__1321 = t___4595.clone();
                break 'ok___11526;
            }
            q__1321 = panic!();
        }
        let mut t___9279: bool = Some(q__1321.to_string().as_str()) == Some("DELETE FROM logs WHERE created_at < '2024-01-01' AND level = 'debug'");
        #[derive(Clone)]
        struct ClosureGroup___141 {}
        impl ClosureGroup___141 {
            fn fn__9265(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete multi where".to_string());
            }
        }
        let closure_group = ClosureGroup___141 {};
        let fn__9265 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9265())
        };
        test___131.assert(t___9279, fn__9265.clone());
        test___131.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryBubblesWithoutWhere__1898() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___132 = temper_std::testing::Test::new();
        let didBubble__1323: bool;
        'ok___11527: {
            'orelse___2042: {
                match delete_from(sid__546("users")).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2042
                };
                didBubble__1323 = false;
                break 'ok___11527;
            }
            didBubble__1323 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___142 {}
        impl ClosureGroup___142 {
            fn fn__9261(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete without WHERE should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___142 {};
        let fn__9261 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9261())
        };
        test___132.assert(didBubble__1323, fn__9261.clone());
        test___132.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryOrWhere__1899() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___133 = temper_std::testing::Test::new();
        let mut t___9246: SafeIdentifier;
        let mut t___9247: SqlBuilder;
        let mut t___9250: SqlFragment;
        let mut t___9251: DeleteQuery;
        let mut t___9252: SqlBuilder;
        let mut t___4574: SqlFragment;
        let q__1325: SqlFragment;
        'ok___11528: {
            'orelse___2043: {
                t___9246 = sid__546("sessions");
                t___9247 = SqlBuilder::new();
                t___9247.append_safe("expired = ");
                t___9247.append_boolean(true);
                t___9250 = t___9247.accumulated();
                t___9251 = delete_from(t___9246.clone()).r#where(t___9250.clone());
                t___9252 = SqlBuilder::new();
                t___9252.append_safe("created_at < ");
                t___9252.append_string("2023-01-01");
                t___4574 = match t___9251.or_where(t___9252.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2043
                };
                q__1325 = t___4574.clone();
                break 'ok___11528;
            }
            q__1325 = panic!();
        }
        let mut t___9259: bool = Some(q__1325.to_string().as_str()) == Some("DELETE FROM sessions WHERE expired = TRUE OR created_at < '2023-01-01'");
        #[derive(Clone)]
        struct ClosureGroup___143 {}
        impl ClosureGroup___143 {
            fn fn__9245(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___143 {};
        let fn__9245 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9245())
        };
        test___133.assert(t___9259, fn__9245.clone());
        test___133.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryWithLimit__1902() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___134 = temper_std::testing::Test::new();
        let mut t___9235: SafeIdentifier;
        let mut t___9236: SqlBuilder;
        let mut t___9239: SqlFragment;
        let mut t___4555: DeleteQuery;
        let mut t___4556: SqlFragment;
        let q__1327: SqlFragment;
        'ok___11529: {
            'orelse___2044: {
                t___9235 = sid__546("logs");
                t___9236 = SqlBuilder::new();
                t___9236.append_safe("level = ");
                t___9236.append_string("debug");
                t___9239 = t___9236.accumulated();
                t___4555 = match delete_from(t___9235.clone()).r#where(t___9239.clone()).limit(1000) {
                    Ok(x) => x,
                    _ => break 'orelse___2044
                };
                t___4556 = match t___4555.to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2044
                };
                q__1327 = t___4556.clone();
                break 'ok___11529;
            }
            q__1327 = panic!();
        }
        let mut t___9243: bool = Some(q__1327.to_string().as_str()) == Some("DELETE FROM logs WHERE level = 'debug' LIMIT 1000");
        #[derive(Clone)]
        struct ClosureGroup___144 {}
        impl ClosureGroup___144 {
            fn fn__9234(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete limit".to_string());
            }
        }
        let closure_group = ClosureGroup___144 {};
        let fn__9234 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9234())
        };
        test___134.assert(t___9243, fn__9234.clone());
        test___134.soft_fail_to_hard()
    }
    #[test]
    fn orderByNullsNullsFirst__1904() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___135 = temper_std::testing::Test::new();
        let mut t___9225: SafeIdentifier = sid__546("users");
        let mut t___9226: SafeIdentifier = sid__546("email");
        let mut t___9227: NullsFirst = NullsFirst::new();
        let q__1329: Query = from(t___9225.clone()).order_by_nulls(t___9226.clone(), true, NullsPosition::new(t___9227.clone()));
        let mut t___9232: bool = Some(q__1329.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY email ASC NULLS FIRST");
        #[derive(Clone)]
        struct ClosureGroup___145 {}
        impl ClosureGroup___145 {
            fn fn__9224(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("nulls first".to_string());
            }
        }
        let closure_group = ClosureGroup___145 {};
        let fn__9224 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9224())
        };
        test___135.assert(t___9232, fn__9224.clone());
        test___135.soft_fail_to_hard()
    }
    #[test]
    fn orderByNullsNullsLast__1905() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___136 = temper_std::testing::Test::new();
        let mut t___9215: SafeIdentifier = sid__546("users");
        let mut t___9216: SafeIdentifier = sid__546("score");
        let mut t___9217: NullsLast = NullsLast::new();
        let q__1331: Query = from(t___9215.clone()).order_by_nulls(t___9216.clone(), false, NullsPosition::new(t___9217.clone()));
        let mut t___9222: bool = Some(q__1331.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY score DESC NULLS LAST");
        #[derive(Clone)]
        struct ClosureGroup___146 {}
        impl ClosureGroup___146 {
            fn fn__9214(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("nulls last".to_string());
            }
        }
        let closure_group = ClosureGroup___146 {};
        let fn__9214 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9214())
        };
        test___136.assert(t___9222, fn__9214.clone());
        test___136.soft_fail_to_hard()
    }
    #[test]
    fn mixedOrderByAndOrderByNulls__1906() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___137 = temper_std::testing::Test::new();
        let mut t___9203: SafeIdentifier = sid__546("users");
        let mut t___9204: SafeIdentifier = sid__546("name");
        let q__1333: Query = from(t___9203.clone()).order_by(t___9204.clone(), true).order_by_nulls(sid__546("email"), true, NullsPosition::new(NullsFirst::new()));
        let mut t___9212: bool = Some(q__1333.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC, email ASC NULLS FIRST");
        #[derive(Clone)]
        struct ClosureGroup___147 {}
        impl ClosureGroup___147 {
            fn fn__9202(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mixed order".to_string());
            }
        }
        let closure_group = ClosureGroup___147 {};
        let fn__9202 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9202())
        };
        test___137.assert(t___9212, fn__9202.clone());
        test___137.soft_fail_to_hard()
    }
    #[test]
    fn crossJoin__1907() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___138 = temper_std::testing::Test::new();
        let mut t___9194: SafeIdentifier = sid__546("users");
        let mut t___9195: SafeIdentifier = sid__546("colors");
        let q__1335: Query = from(t___9194.clone()).cross_join(t___9195.clone());
        let mut t___9200: bool = Some(q__1335.to_sql().to_string().as_str()) == Some("SELECT * FROM users CROSS JOIN colors");
        #[derive(Clone)]
        struct ClosureGroup___148 {}
        impl ClosureGroup___148 {
            fn fn__9193(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("cross join".to_string());
            }
        }
        let closure_group = ClosureGroup___148 {};
        let fn__9193 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9193())
        };
        test___138.assert(t___9200, fn__9193.clone());
        test___138.soft_fail_to_hard()
    }
    #[test]
    fn crossJoinCombinedWithOtherJoins__1908() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___139 = temper_std::testing::Test::new();
        let mut t___9180: SafeIdentifier = sid__546("users");
        let mut t___9181: SafeIdentifier = sid__546("orders");
        let mut t___9182: SqlBuilder = SqlBuilder::new();
        t___9182.append_safe("users.id = orders.user_id");
        let mut t___9184: SqlFragment = t___9182.accumulated();
        let q__1337: Query = from(t___9180.clone()).inner_join(t___9181.clone(), t___9184.clone()).cross_join(sid__546("colors"));
        let mut t___9191: bool = Some(q__1337.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id CROSS JOIN colors");
        #[derive(Clone)]
        struct ClosureGroup___149 {}
        impl ClosureGroup___149 {
            fn fn__9179(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("cross + inner join".to_string());
            }
        }
        let closure_group = ClosureGroup___149 {};
        let fn__9179 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9179())
        };
        test___139.assert(t___9191, fn__9179.clone());
        test___139.soft_fail_to_hard()
    }
    #[test]
    fn lockForUpdate__1910() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___140 = temper_std::testing::Test::new();
        let mut t___9166: SafeIdentifier = sid__546("users");
        let mut t___9167: SqlBuilder = SqlBuilder::new();
        t___9167.append_safe("id = ");
        t___9167.append_int32(1);
        let mut t___9170: SqlFragment = t___9167.accumulated();
        let q__1339: Query = from(t___9166.clone()).r#where(t___9170.clone()).lock(LockMode::new(ForUpdate::new()));
        let mut t___9177: bool = Some(q__1339.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id = 1 FOR UPDATE");
        #[derive(Clone)]
        struct ClosureGroup___150 {}
        impl ClosureGroup___150 {
            fn fn__9165(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("for update".to_string());
            }
        }
        let closure_group = ClosureGroup___150 {};
        let fn__9165 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9165())
        };
        test___140.assert(t___9177, fn__9165.clone());
        test___140.soft_fail_to_hard()
    }
    #[test]
    fn lockForShare__1912() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___141 = temper_std::testing::Test::new();
        let mut t___9155: SafeIdentifier = sid__546("users");
        let mut t___9156: SafeIdentifier = sid__546("name");
        let q__1341: Query = from(t___9155.clone()).select([t___9156.clone()]).lock(LockMode::new(ForShare::new()));
        let mut t___9163: bool = Some(q__1341.to_sql().to_string().as_str()) == Some("SELECT name FROM users FOR SHARE");
        #[derive(Clone)]
        struct ClosureGroup___151 {}
        impl ClosureGroup___151 {
            fn fn__9154(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("for share".to_string());
            }
        }
        let closure_group = ClosureGroup___151 {};
        let fn__9154 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9154())
        };
        test___141.assert(t___9163, fn__9154.clone());
        test___141.soft_fail_to_hard()
    }
    #[test]
    fn lockWithFullQuery__1913() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___142 = temper_std::testing::Test::new();
        let mut t___9141: SafeIdentifier;
        let mut t___9142: SqlBuilder;
        let mut t___9145: SqlFragment;
        let mut t___9148: Query;
        let mut t___4479: Query;
        let q__1343: Query;
        'ok___11530: {
            'orelse___2045: {
                t___9141 = sid__546("accounts");
                t___9142 = SqlBuilder::new();
                t___9142.append_safe("id = ");
                t___9142.append_int32(42);
                t___9145 = t___9142.accumulated();
                t___4479 = match from(t___9141.clone()).r#where(t___9145.clone()).limit(1) {
                    Ok(x) => x,
                    _ => break 'orelse___2045
                };
                t___9148 = t___4479.lock(LockMode::new(ForUpdate::new()));
                q__1343 = t___9148.clone();
                break 'ok___11530;
            }
            q__1343 = panic!();
        }
        let mut t___9152: bool = Some(q__1343.to_sql().to_string().as_str()) == Some("SELECT * FROM accounts WHERE id = 42 LIMIT 1 FOR UPDATE");
        #[derive(Clone)]
        struct ClosureGroup___152 {}
        impl ClosureGroup___152 {
            fn fn__9140(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("lock full query".to_string());
            }
        }
        let closure_group = ClosureGroup___152 {};
        let fn__9140 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9140())
        };
        test___142.assert(t___9152, fn__9140.clone());
        test___142.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierAcceptsValidNames__1915() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___149 = temper_std::testing::Test::new();
        let mut t___4468: SafeIdentifier;
        let id__1381: SafeIdentifier;
        'ok___11531: {
            'orelse___2046: {
                t___4468 = match safe_identifier("user_name") {
                    Ok(x) => x,
                    _ => break 'orelse___2046
                };
                id__1381 = t___4468.clone();
                break 'ok___11531;
            }
            id__1381 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___9138: bool = Some(id__1381.sql_value().as_str()) == Some("user_name");
        #[derive(Clone)]
        struct ClosureGroup___153 {}
        impl ClosureGroup___153 {
            fn fn__9135(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("value should round-trip".to_string());
            }
        }
        let closure_group = ClosureGroup___153 {};
        let fn__9135 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9135())
        };
        test___149.assert(t___9138, fn__9135.clone());
        test___149.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsEmptyString__1916() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___150 = temper_std::testing::Test::new();
        let didBubble__1383: bool;
        'ok___11532: {
            'orelse___2047: {
                match safe_identifier("") {
                    Ok(x) => x,
                    _ => break 'orelse___2047
                };
                didBubble__1383 = false;
                break 'ok___11532;
            }
            didBubble__1383 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___154 {}
        impl ClosureGroup___154 {
            fn fn__9132(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty string should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___154 {};
        let fn__9132 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9132())
        };
        test___150.assert(didBubble__1383, fn__9132.clone());
        test___150.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsLeadingDigit__1917() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___151 = temper_std::testing::Test::new();
        let didBubble__1385: bool;
        'ok___11533: {
            'orelse___2048: {
                match safe_identifier("1col") {
                    Ok(x) => x,
                    _ => break 'orelse___2048
                };
                didBubble__1385 = false;
                break 'ok___11533;
            }
            didBubble__1385 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___155 {}
        impl ClosureGroup___155 {
            fn fn__9129(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("leading digit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___155 {};
        let fn__9129 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9129())
        };
        test___151.assert(didBubble__1385, fn__9129.clone());
        test___151.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsSqlMetacharacters__1918() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___152 = temper_std::testing::Test::new();
        let cases__1387: temper_core::List<std::sync::Arc<String>> = std::sync::Arc::new(vec![std::sync::Arc::new("name); DROP TABLE".to_string()), std::sync::Arc::new("col'".to_string()), std::sync::Arc::new("a b".to_string()), std::sync::Arc::new("a-b".to_string()), std::sync::Arc::new("a.b".to_string()), std::sync::Arc::new("a;b".to_string())]);
        #[derive(Clone)]
        struct ClosureGroup___156 {
            test___152: temper_std::testing::Test
        }
        impl ClosureGroup___156 {
            fn fn__9126(& self, c__1388: impl temper_core::ToArcString) {
                let c__1388 = c__1388.to_arc_string();
                let didBubble__1389: bool;
                'ok___11534: {
                    'orelse___2049: {
                        match safe_identifier(c__1388.clone()) {
                            Ok(x) => x,
                            _ => break 'orelse___2049
                        };
                        didBubble__1389 = false;
                        break 'ok___11534;
                    }
                    didBubble__1389 = true;
                }
                #[derive(Clone)]
                struct ClosureGroup___157 {
                    c__1388: std::sync::Arc<String>
                }
                impl ClosureGroup___157 {
                    fn fn__9123(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject: {}", self.c__1388.clone()));
                    }
                }
                let closure_group = ClosureGroup___157 {
                    c__1388: c__1388.clone()
                };
                let fn__9123 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__9123())
                };
                self.test___152.assert(didBubble__1389, fn__9123.clone());
            }
        }
        let closure_group = ClosureGroup___156 {
            test___152: test___152.clone()
        };
        let fn__9126 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1388: std::sync::Arc<String> | closure_group.fn__9126(c__1388))
        };
        temper_core::listed::list_for_each( & cases__1387, & ( * fn__9126.clone()));
        test___152.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupFound__1919() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___153 = temper_std::testing::Test::new();
        let mut t___4445: SafeIdentifier;
        let mut t___4446: SafeIdentifier;
        let mut t___4447: SafeIdentifier;
        let mut t___4448: SafeIdentifier;
        let mut t___4451: SafeIdentifier;
        let mut t___4452: SafeIdentifier;
        let mut t___4456: FieldDef;
        'ok___11535: {
            'orelse___2050: {
                t___4445 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___2050
                };
                t___4446 = t___4445.clone();
                break 'ok___11535;
            }
            t___4446 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___11536: {
            'orelse___2051: {
                t___4447 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___2051
                };
                t___4448 = t___4447.clone();
                break 'ok___11536;
            }
            t___4448 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___9113: StringField = StringField::new();
        let mut t___9114: FieldDef = FieldDef::new(t___4448.clone(), FieldType::new(t___9113.clone()), false);
        'ok___11537: {
            'orelse___2052: {
                t___4451 = match safe_identifier("age") {
                    Ok(x) => x,
                    _ => break 'orelse___2052
                };
                t___4452 = t___4451.clone();
                break 'ok___11537;
            }
            t___4452 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___9115: IntField = IntField::new();
        let mut t___9116: FieldDef = FieldDef::new(t___4452.clone(), FieldType::new(t___9115.clone()), false);
        let td__1391: TableDef = TableDef::new(t___4446.clone(), [t___9114.clone(), t___9116.clone()]);
        let f__1392: FieldDef;
        'ok___11538: {
            'orelse___2053: {
                t___4456 = match td__1391.field("age") {
                    Ok(x) => x,
                    _ => break 'orelse___2053
                };
                f__1392 = t___4456.clone();
                break 'ok___11538;
            }
            f__1392 = panic!();
        }
        let mut t___9121: bool = Some(f__1392.name().sql_value().as_str()) == Some("age");
        #[derive(Clone)]
        struct ClosureGroup___158 {}
        impl ClosureGroup___158 {
            fn fn__9112(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should find age field".to_string());
            }
        }
        let closure_group = ClosureGroup___158 {};
        let fn__9112 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9112())
        };
        test___153.assert(t___9121, fn__9112.clone());
        test___153.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupNotFoundBubbles__1920() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___154 = temper_std::testing::Test::new();
        let mut t___4436: SafeIdentifier;
        let mut t___4437: SafeIdentifier;
        let mut t___4438: SafeIdentifier;
        let mut t___4439: SafeIdentifier;
        'ok___11539: {
            'orelse___2054: {
                t___4436 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___2054
                };
                t___4437 = t___4436.clone();
                break 'ok___11539;
            }
            t___4437 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___11540: {
            'orelse___2055: {
                t___4438 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___2055
                };
                t___4439 = t___4438.clone();
                break 'ok___11540;
            }
            t___4439 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___9107: StringField = StringField::new();
        let mut t___9108: FieldDef = FieldDef::new(t___4439.clone(), FieldType::new(t___9107.clone()), false);
        let td__1394: TableDef = TableDef::new(t___4437.clone(), [t___9108.clone()]);
        let didBubble__1395: bool;
        'ok___11541: {
            'orelse___2056: {
                match td__1394.field("nonexistent") {
                    Ok(x) => x,
                    _ => break 'orelse___2056
                };
                didBubble__1395 = false;
                break 'ok___11541;
            }
            didBubble__1395 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___159 {}
        impl ClosureGroup___159 {
            fn fn__9106(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("unknown field should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___159 {};
        let fn__9106 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9106())
        };
        test___154.assert(didBubble__1395, fn__9106.clone());
        test___154.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefNullableFlag__1921() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___155 = temper_std::testing::Test::new();
        let mut t___4424: SafeIdentifier;
        let mut t___4425: SafeIdentifier;
        let mut t___4428: SafeIdentifier;
        let mut t___4429: SafeIdentifier;
        'ok___11542: {
            'orelse___2057: {
                t___4424 = match safe_identifier("email") {
                    Ok(x) => x,
                    _ => break 'orelse___2057
                };
                t___4425 = t___4424.clone();
                break 'ok___11542;
            }
            t___4425 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___9095: StringField = StringField::new();
        let required__1397: FieldDef = FieldDef::new(t___4425.clone(), FieldType::new(t___9095.clone()), false);
        'ok___11543: {
            'orelse___2058: {
                t___4428 = match safe_identifier("bio") {
                    Ok(x) => x,
                    _ => break 'orelse___2058
                };
                t___4429 = t___4428.clone();
                break 'ok___11543;
            }
            t___4429 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___9097: StringField = StringField::new();
        let optional__1398: FieldDef = FieldDef::new(t___4429.clone(), FieldType::new(t___9097.clone()), true);
        let mut t___9101: bool = ! required__1397.nullable();
        #[derive(Clone)]
        struct ClosureGroup___160 {}
        impl ClosureGroup___160 {
            fn fn__9094(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("required field should not be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___160 {};
        let fn__9094 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9094())
        };
        test___155.assert(t___9101, fn__9094.clone());
        let mut t___9103: bool = optional__1398.nullable();
        #[derive(Clone)]
        struct ClosureGroup___161 {}
        impl ClosureGroup___161 {
            fn fn__9093(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("optional field should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___161 {};
        let fn__9093 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9093())
        };
        test___155.assert(t___9103, fn__9093.clone());
        test___155.soft_fail_to_hard()
    }
    #[test]
    fn stringEscaping__1922() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___159 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___162 {}
        impl ClosureGroup___162 {
            fn build__1524(& self, name__1526: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__1526 = name__1526.to_arc_string();
                let mut t___9075: SqlBuilder = SqlBuilder::new();
                t___9075.append_safe("select * from hi where name = ");
                t___9075.append_string(name__1526.clone());
                return t___9075.accumulated().to_string();
            }
            fn buildWrong__1525(& self, name__1528: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__1528 = name__1528.to_arc_string();
                return std::sync::Arc::new(format!("select * from hi where name = '{}'", name__1528.clone()));
            }
        }
        let closure_group = ClosureGroup___162 {};
        let build__1524 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__1526: std::sync::Arc<String> | closure_group.build__1524(name__1526))
        };
        let buildWrong__1525 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__1528: std::sync::Arc<String> | closure_group.buildWrong__1525(name__1528))
        };
        let actual___1924: std::sync::Arc<String> = build__1524(std::sync::Arc::new("world".to_string()));
        let mut t___9085: bool = Some(actual___1924.as_str()) == Some("select * from hi where name = 'world'");
        #[derive(Clone)]
        struct ClosureGroup___163 {
            actual___1924: std::sync::Arc<String>
        }
        impl ClosureGroup___163 {
            fn fn__9082(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(\"world\") == (select * from hi where name = 'world') not ({})", self.actual___1924.clone()));
            }
        }
        let closure_group = ClosureGroup___163 {
            actual___1924: actual___1924.clone()
        };
        let fn__9082 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9082())
        };
        test___159.assert(t___9085, fn__9082.clone());
        let bobbyTables__1530: std::sync::Arc<String> = std::sync::Arc::new("Robert'); drop table hi;--".to_string());
        let actual___1926: std::sync::Arc<String> = build__1524(std::sync::Arc::new("Robert'); drop table hi;--".to_string()));
        let mut t___9089: bool = Some(actual___1926.as_str()) == Some("select * from hi where name = 'Robert''); drop table hi;--'");
        #[derive(Clone)]
        struct ClosureGroup___164 {
            actual___1926: std::sync::Arc<String>
        }
        impl ClosureGroup___164 {
            fn fn__9081(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(bobbyTables) == (select * from hi where name = 'Robert''); drop table hi;--') not ({})", self.actual___1926.clone()));
            }
        }
        let closure_group = ClosureGroup___164 {
            actual___1926: actual___1926.clone()
        };
        let fn__9081 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9081())
        };
        test___159.assert(t___9089, fn__9081.clone());
        #[derive(Clone)]
        struct ClosureGroup___165 {}
        impl ClosureGroup___165 {
            fn fn__9080(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')".to_string());
            }
        }
        let closure_group = ClosureGroup___165 {};
        let fn__9080 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9080())
        };
        test___159.assert(true, fn__9080.clone());
        test___159.soft_fail_to_hard()
    }
    #[test]
    fn stringEdgeCases__1930() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___160 = temper_std::testing::Test::new();
        let mut t___9043: SqlBuilder = SqlBuilder::new();
        t___9043.append_safe("v = ");
        t___9043.append_string("");
        let actual___1931: std::sync::Arc<String> = t___9043.accumulated().to_string();
        let mut t___9049: bool = Some(actual___1931.as_str()) == Some("v = ''");
        #[derive(Clone)]
        struct ClosureGroup___166 {
            actual___1931: std::sync::Arc<String>
        }
        impl ClosureGroup___166 {
            fn fn__9042(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"\").toString() == (v = '') not ({})", self.actual___1931.clone()));
            }
        }
        let closure_group = ClosureGroup___166 {
            actual___1931: actual___1931.clone()
        };
        let fn__9042 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9042())
        };
        test___160.assert(t___9049, fn__9042.clone());
        let mut t___9051: SqlBuilder = SqlBuilder::new();
        t___9051.append_safe("v = ");
        t___9051.append_string("a''b");
        let actual___1934: std::sync::Arc<String> = t___9051.accumulated().to_string();
        let mut t___9057: bool = Some(actual___1934.as_str()) == Some("v = 'a''''b'");
        #[derive(Clone)]
        struct ClosureGroup___167 {
            actual___1934: std::sync::Arc<String>
        }
        impl ClosureGroup___167 {
            fn fn__9041(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (v = 'a''''b') not ({})", self.actual___1934.clone()));
            }
        }
        let closure_group = ClosureGroup___167 {
            actual___1934: actual___1934.clone()
        };
        let fn__9041 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9041())
        };
        test___160.assert(t___9057, fn__9041.clone());
        let mut t___9059: SqlBuilder = SqlBuilder::new();
        t___9059.append_safe("v = ");
        t___9059.append_string("Hello 世界");
        let actual___1937: std::sync::Arc<String> = t___9059.accumulated().to_string();
        let mut t___9065: bool = Some(actual___1937.as_str()) == Some("v = 'Hello 世界'");
        #[derive(Clone)]
        struct ClosureGroup___168 {
            actual___1937: std::sync::Arc<String>
        }
        impl ClosureGroup___168 {
            fn fn__9040(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Hello 世界\").toString() == (v = 'Hello 世界') not ({})", self.actual___1937.clone()));
            }
        }
        let closure_group = ClosureGroup___168 {
            actual___1937: actual___1937.clone()
        };
        let fn__9040 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9040())
        };
        test___160.assert(t___9065, fn__9040.clone());
        let mut t___9067: SqlBuilder = SqlBuilder::new();
        t___9067.append_safe("v = ");
        t___9067.append_string("Line1\x0aLine2");
        let actual___1940: std::sync::Arc<String> = t___9067.accumulated().to_string();
        let mut t___9073: bool = Some(actual___1940.as_str()) == Some("v = 'Line1\x0aLine2'");
        #[derive(Clone)]
        struct ClosureGroup___169 {
            actual___1940: std::sync::Arc<String>
        }
        impl ClosureGroup___169 {
            fn fn__9039(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Line1\\nLine2\").toString() == (v = 'Line1\x0aLine2') not ({})", self.actual___1940.clone()));
            }
        }
        let closure_group = ClosureGroup___169 {
            actual___1940: actual___1940.clone()
        };
        let fn__9039 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9039())
        };
        test___160.assert(t___9073, fn__9039.clone());
        test___160.soft_fail_to_hard()
    }
    #[test]
    fn numbersAndBooleans__1943() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___161 = temper_std::testing::Test::new();
        let mut t___4369: temper_std::temporal::Date;
        let mut t___9014: SqlBuilder = SqlBuilder::new();
        t___9014.append_safe("select ");
        t___9014.append_int32(42);
        t___9014.append_safe(", ");
        t___9014.append_int64(43);
        t___9014.append_safe(", ");
        t___9014.append_float64(19.99f64);
        t___9014.append_safe(", ");
        t___9014.append_boolean(true);
        t___9014.append_safe(", ");
        t___9014.append_boolean(false);
        let actual___1944: std::sync::Arc<String> = t___9014.accumulated().to_string();
        let mut t___9028: bool = Some(actual___1944.as_str()) == Some("select 42, 43, 19.99, TRUE, FALSE");
        #[derive(Clone)]
        struct ClosureGroup___170 {
            actual___1944: std::sync::Arc<String>
        }
        impl ClosureGroup___170 {
            fn fn__9013(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, 42, \", \", \\interpolate, 43, \", \", \\interpolate, 19.99, \", \", \\interpolate, true, \", \", \\interpolate, false).toString() == (select 42, 43, 19.99, TRUE, FALSE) not ({})", self.actual___1944.clone()));
            }
        }
        let closure_group = ClosureGroup___170 {
            actual___1944: actual___1944.clone()
        };
        let fn__9013 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9013())
        };
        test___161.assert(t___9028, fn__9013.clone());
        let date__1533: temper_std::temporal::Date;
        'ok___11544: {
            'orelse___2059: {
                t___4369 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___2059
                };
                date__1533 = t___4369.clone();
                break 'ok___11544;
            }
            date__1533 = panic!();
        }
        let mut t___9030: SqlBuilder = SqlBuilder::new();
        t___9030.append_safe("insert into t values (");
        t___9030.append_date(date__1533.clone());
        t___9030.append_safe(")");
        let actual___1947: std::sync::Arc<String> = t___9030.accumulated().to_string();
        let mut t___9037: bool = Some(actual___1947.as_str()) == Some("insert into t values ('2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___171 {
            actual___1947: std::sync::Arc<String>
        }
        impl ClosureGroup___171 {
            fn fn__9012(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"insert into t values (\", \\interpolate, date, \")\").toString() == (insert into t values ('2024-12-25')) not ({})", self.actual___1947.clone()));
            }
        }
        let closure_group = ClosureGroup___171 {
            actual___1947: actual___1947.clone()
        };
        let fn__9012 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9012())
        };
        test___161.assert(t___9037, fn__9012.clone());
        test___161.soft_fail_to_hard()
    }
    #[test]
    fn lists__1950() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___162 = temper_std::testing::Test::new();
        let mut t___4341: temper_std::temporal::Date;
        let mut t___4342: temper_std::temporal::Date;
        let mut t___4343: temper_std::temporal::Date;
        let mut t___4344: temper_std::temporal::Date;
        let mut t___8958: SqlBuilder = SqlBuilder::new();
        t___8958.append_safe("v IN (");
        t___8958.append_string_list(temper_core::ToListed::to_listed([std::sync::Arc::new("a".to_string()), std::sync::Arc::new("b".to_string()), std::sync::Arc::new("c'd".to_string())]));
        t___8958.append_safe(")");
        let actual___1951: std::sync::Arc<String> = t___8958.accumulated().to_string();
        let mut t___8965: bool = Some(actual___1951.as_str()) == Some("v IN ('a', 'b', 'c''d')");
        #[derive(Clone)]
        struct ClosureGroup___172 {
            actual___1951: std::sync::Arc<String>
        }
        impl ClosureGroup___172 {
            fn fn__8957(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (v IN ('a', 'b', 'c''d')) not ({})", self.actual___1951.clone()));
            }
        }
        let closure_group = ClosureGroup___172 {
            actual___1951: actual___1951.clone()
        };
        let fn__8957 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8957())
        };
        test___162.assert(t___8965, fn__8957.clone());
        let mut t___8967: SqlBuilder = SqlBuilder::new();
        t___8967.append_safe("v IN (");
        t___8967.append_int32_list(temper_core::ToListed::to_listed([1, 2, 3]));
        t___8967.append_safe(")");
        let actual___1954: std::sync::Arc<String> = t___8967.accumulated().to_string();
        let mut t___8974: bool = Some(actual___1954.as_str()) == Some("v IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___173 {
            actual___1954: std::sync::Arc<String>
        }
        impl ClosureGroup___173 {
            fn fn__8956(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2, 3), \")\").toString() == (v IN (1, 2, 3)) not ({})", self.actual___1954.clone()));
            }
        }
        let closure_group = ClosureGroup___173 {
            actual___1954: actual___1954.clone()
        };
        let fn__8956 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8956())
        };
        test___162.assert(t___8974, fn__8956.clone());
        let mut t___8976: SqlBuilder = SqlBuilder::new();
        t___8976.append_safe("v IN (");
        t___8976.append_int64_list(temper_core::ToListed::to_listed([1, 2]));
        t___8976.append_safe(")");
        let actual___1957: std::sync::Arc<String> = t___8976.accumulated().to_string();
        let mut t___8983: bool = Some(actual___1957.as_str()) == Some("v IN (1, 2)");
        #[derive(Clone)]
        struct ClosureGroup___174 {
            actual___1957: std::sync::Arc<String>
        }
        impl ClosureGroup___174 {
            fn fn__8955(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2), \")\").toString() == (v IN (1, 2)) not ({})", self.actual___1957.clone()));
            }
        }
        let closure_group = ClosureGroup___174 {
            actual___1957: actual___1957.clone()
        };
        let fn__8955 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8955())
        };
        test___162.assert(t___8983, fn__8955.clone());
        let mut t___8985: SqlBuilder = SqlBuilder::new();
        t___8985.append_safe("v IN (");
        t___8985.append_float64_list(temper_core::ToListed::to_listed([1.0f64, 2.0f64]));
        t___8985.append_safe(")");
        let actual___1960: std::sync::Arc<String> = t___8985.accumulated().to_string();
        let mut t___8992: bool = Some(actual___1960.as_str()) == Some("v IN (1.0, 2.0)");
        #[derive(Clone)]
        struct ClosureGroup___175 {
            actual___1960: std::sync::Arc<String>
        }
        impl ClosureGroup___175 {
            fn fn__8954(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1.0, 2.0), \")\").toString() == (v IN (1.0, 2.0)) not ({})", self.actual___1960.clone()));
            }
        }
        let closure_group = ClosureGroup___175 {
            actual___1960: actual___1960.clone()
        };
        let fn__8954 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8954())
        };
        test___162.assert(t___8992, fn__8954.clone());
        let mut t___8994: SqlBuilder = SqlBuilder::new();
        t___8994.append_safe("v IN (");
        t___8994.append_boolean_list(temper_core::ToListed::to_listed([true, false]));
        t___8994.append_safe(")");
        let actual___1963: std::sync::Arc<String> = t___8994.accumulated().to_string();
        let mut t___9001: bool = Some(actual___1963.as_str()) == Some("v IN (TRUE, FALSE)");
        #[derive(Clone)]
        struct ClosureGroup___176 {
            actual___1963: std::sync::Arc<String>
        }
        impl ClosureGroup___176 {
            fn fn__8953(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(true, false), \")\").toString() == (v IN (TRUE, FALSE)) not ({})", self.actual___1963.clone()));
            }
        }
        let closure_group = ClosureGroup___176 {
            actual___1963: actual___1963.clone()
        };
        let fn__8953 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8953())
        };
        test___162.assert(t___9001, fn__8953.clone());
        'ok___11545: {
            'orelse___2060: {
                t___4341 = match temper_std::temporal::Date::new(2024, 1, 1) {
                    Ok(x) => x,
                    _ => break 'orelse___2060
                };
                t___4342 = t___4341.clone();
                break 'ok___11545;
            }
            t___4342 = panic!();
        }
        'ok___11546: {
            'orelse___2061: {
                t___4343 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___2061
                };
                t___4344 = t___4343.clone();
                break 'ok___11546;
            }
            t___4344 = panic!();
        }
        let dates__1535: temper_core::List<temper_std::temporal::Date> = std::sync::Arc::new(vec![t___4342.clone(), t___4344.clone()]);
        let mut t___9003: SqlBuilder = SqlBuilder::new();
        t___9003.append_safe("v IN (");
        t___9003.append_date_list(temper_core::ToListed::to_listed(dates__1535.clone()));
        t___9003.append_safe(")");
        let actual___1966: std::sync::Arc<String> = t___9003.accumulated().to_string();
        let mut t___9010: bool = Some(actual___1966.as_str()) == Some("v IN ('2024-01-01', '2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___177 {
            actual___1966: std::sync::Arc<String>
        }
        impl ClosureGroup___177 {
            fn fn__8952(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, dates, \")\").toString() == (v IN ('2024-01-01', '2024-12-25')) not ({})", self.actual___1966.clone()));
            }
        }
        let closure_group = ClosureGroup___177 {
            actual___1966: actual___1966.clone()
        };
        let fn__8952 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8952())
        };
        test___162.assert(t___9010, fn__8952.clone());
        test___162.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_naNRendersAsNull__1969() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___163 = temper_std::testing::Test::new();
        let nan__1537: f64;
        nan__1537 = temper_core::float64::div(0.0f64, 0.0f64) ? ;
        let mut t___8944: SqlBuilder = SqlBuilder::new();
        t___8944.append_safe("v = ");
        t___8944.append_float64(nan__1537);
        let actual___1970: std::sync::Arc<String> = t___8944.accumulated().to_string();
        let mut t___8950: bool = Some(actual___1970.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___178 {
            actual___1970: std::sync::Arc<String>
        }
        impl ClosureGroup___178 {
            fn fn__8943(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, nan).toString() == (v = NULL) not ({})", self.actual___1970.clone()));
            }
        }
        let closure_group = ClosureGroup___178 {
            actual___1970: actual___1970.clone()
        };
        let fn__8943 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8943())
        };
        test___163.assert(t___8950, fn__8943.clone());
        test___163.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_infinityRendersAsNull__1973() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___164 = temper_std::testing::Test::new();
        let inf__1539: f64;
        inf__1539 = temper_core::float64::div(1.0f64, 0.0f64) ? ;
        let mut t___8935: SqlBuilder = SqlBuilder::new();
        t___8935.append_safe("v = ");
        t___8935.append_float64(inf__1539);
        let actual___1974: std::sync::Arc<String> = t___8935.accumulated().to_string();
        let mut t___8941: bool = Some(actual___1974.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___179 {
            actual___1974: std::sync::Arc<String>
        }
        impl ClosureGroup___179 {
            fn fn__8934(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, inf).toString() == (v = NULL) not ({})", self.actual___1974.clone()));
            }
        }
        let closure_group = ClosureGroup___179 {
            actual___1974: actual___1974.clone()
        };
        let fn__8934 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8934())
        };
        test___164.assert(t___8941, fn__8934.clone());
        test___164.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_negativeInfinityRendersAsNull__1977() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___165 = temper_std::testing::Test::new();
        let ninf__1541: f64;
        ninf__1541 = temper_core::float64::div(-1.0f64, 0.0f64) ? ;
        let mut t___8926: SqlBuilder = SqlBuilder::new();
        t___8926.append_safe("v = ");
        t___8926.append_float64(ninf__1541);
        let actual___1978: std::sync::Arc<String> = t___8926.accumulated().to_string();
        let mut t___8932: bool = Some(actual___1978.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___180 {
            actual___1978: std::sync::Arc<String>
        }
        impl ClosureGroup___180 {
            fn fn__8925(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, ninf).toString() == (v = NULL) not ({})", self.actual___1978.clone()));
            }
        }
        let closure_group = ClosureGroup___180 {
            actual___1978: actual___1978.clone()
        };
        let fn__8925 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8925())
        };
        test___165.assert(t___8932, fn__8925.clone());
        test___165.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_normalValuesStillWork__1981() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___166 = temper_std::testing::Test::new();
        let mut t___8901: SqlBuilder = SqlBuilder::new();
        t___8901.append_safe("v = ");
        t___8901.append_float64(3.14f64);
        let actual___1982: std::sync::Arc<String> = t___8901.accumulated().to_string();
        let mut t___8907: bool = Some(actual___1982.as_str()) == Some("v = 3.14");
        #[derive(Clone)]
        struct ClosureGroup___181 {
            actual___1982: std::sync::Arc<String>
        }
        impl ClosureGroup___181 {
            fn fn__8900(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 3.14).toString() == (v = 3.14) not ({})", self.actual___1982.clone()));
            }
        }
        let closure_group = ClosureGroup___181 {
            actual___1982: actual___1982.clone()
        };
        let fn__8900 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8900())
        };
        test___166.assert(t___8907, fn__8900.clone());
        let mut t___8909: SqlBuilder = SqlBuilder::new();
        t___8909.append_safe("v = ");
        t___8909.append_float64(0.0f64);
        let actual___1985: std::sync::Arc<String> = t___8909.accumulated().to_string();
        let mut t___8915: bool = Some(actual___1985.as_str()) == Some("v = 0.0");
        #[derive(Clone)]
        struct ClosureGroup___182 {
            actual___1985: std::sync::Arc<String>
        }
        impl ClosureGroup___182 {
            fn fn__8899(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 0.0).toString() == (v = 0.0) not ({})", self.actual___1985.clone()));
            }
        }
        let closure_group = ClosureGroup___182 {
            actual___1985: actual___1985.clone()
        };
        let fn__8899 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8899())
        };
        test___166.assert(t___8915, fn__8899.clone());
        let mut t___8917: SqlBuilder = SqlBuilder::new();
        t___8917.append_safe("v = ");
        t___8917.append_float64(-42.5f64);
        let actual___1988: std::sync::Arc<String> = t___8917.accumulated().to_string();
        let mut t___8923: bool = Some(actual___1988.as_str()) == Some("v = -42.5");
        #[derive(Clone)]
        struct ClosureGroup___183 {
            actual___1988: std::sync::Arc<String>
        }
        impl ClosureGroup___183 {
            fn fn__8898(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, -42.5).toString() == (v = -42.5) not ({})", self.actual___1988.clone()));
            }
        }
        let closure_group = ClosureGroup___183 {
            actual___1988: actual___1988.clone()
        };
        let fn__8898 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8898())
        };
        test___166.assert(t___8923, fn__8898.clone());
        test___166.soft_fail_to_hard()
    }
    #[test]
    fn sqlDateRendersWithQuotes__1991() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___167 = temper_std::testing::Test::new();
        let mut t___4237: temper_std::temporal::Date;
        let d__1544: temper_std::temporal::Date;
        'ok___11547: {
            'orelse___2062: {
                t___4237 = match temper_std::temporal::Date::new(2024, 6, 15) {
                    Ok(x) => x,
                    _ => break 'orelse___2062
                };
                d__1544 = t___4237.clone();
                break 'ok___11547;
            }
            d__1544 = panic!();
        }
        let mut t___8890: SqlBuilder = SqlBuilder::new();
        t___8890.append_safe("v = ");
        t___8890.append_date(d__1544.clone());
        let actual___1992: std::sync::Arc<String> = t___8890.accumulated().to_string();
        let mut t___8896: bool = Some(actual___1992.as_str()) == Some("v = '2024-06-15'");
        #[derive(Clone)]
        struct ClosureGroup___184 {
            actual___1992: std::sync::Arc<String>
        }
        impl ClosureGroup___184 {
            fn fn__8889(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, d).toString() == (v = '2024-06-15') not ({})", self.actual___1992.clone()));
            }
        }
        let closure_group = ClosureGroup___184 {
            actual___1992: actual___1992.clone()
        };
        let fn__8889 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8889())
        };
        test___167.assert(t___8896, fn__8889.clone());
        test___167.soft_fail_to_hard()
    }
    #[test]
    fn nesting__1995() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___168 = temper_std::testing::Test::new();
        let name__1546: std::sync::Arc<String> = std::sync::Arc::new("Someone".to_string());
        let mut t___8858: SqlBuilder = SqlBuilder::new();
        t___8858.append_safe("where p.last_name = ");
        t___8858.append_string("Someone");
        let condition__1547: SqlFragment = t___8858.accumulated();
        let mut t___8862: SqlBuilder = SqlBuilder::new();
        t___8862.append_safe("select p.id from person p ");
        t___8862.append_fragment(condition__1547.clone());
        let actual___1997: std::sync::Arc<String> = t___8862.accumulated().to_string();
        let mut t___8868: bool = Some(actual___1997.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___185 {
            actual___1997: std::sync::Arc<String>
        }
        impl ClosureGroup___185 {
            fn fn__8857(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1997.clone()));
            }
        }
        let closure_group = ClosureGroup___185 {
            actual___1997: actual___1997.clone()
        };
        let fn__8857 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8857())
        };
        test___168.assert(t___8868, fn__8857.clone());
        let mut t___8870: SqlBuilder = SqlBuilder::new();
        t___8870.append_safe("select p.id from person p ");
        t___8870.append_part(SqlPart::new(condition__1547.to_source()));
        let actual___2000: std::sync::Arc<String> = t___8870.accumulated().to_string();
        let mut t___8877: bool = Some(actual___2000.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___186 {
            actual___2000: std::sync::Arc<String>
        }
        impl ClosureGroup___186 {
            fn fn__8856(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition.toSource()).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___2000.clone()));
            }
        }
        let closure_group = ClosureGroup___186 {
            actual___2000: actual___2000.clone()
        };
        let fn__8856 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8856())
        };
        test___168.assert(t___8877, fn__8856.clone());
        let parts__1548: temper_core::List<SqlPart> = std::sync::Arc::new(vec![SqlPart::new(SqlString::new("a'b")), SqlPart::new(SqlInt32::new(3))]);
        let mut t___8881: SqlBuilder = SqlBuilder::new();
        t___8881.append_safe("select ");
        t___8881.append_part_list(parts__1548.clone());
        let actual___2003: std::sync::Arc<String> = t___8881.accumulated().to_string();
        let mut t___8887: bool = Some(actual___2003.as_str()) == Some("select 'a''b', 3");
        #[derive(Clone)]
        struct ClosureGroup___187 {
            actual___2003: std::sync::Arc<String>
        }
        impl ClosureGroup___187 {
            fn fn__8855(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, parts).toString() == (select 'a''b', 3) not ({})", self.actual___2003.clone()));
            }
        }
        let closure_group = ClosureGroup___187 {
            actual___2003: actual___2003.clone()
        };
        let fn__8855 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8855())
        };
        test___168.assert(t___8887, fn__8855.clone());
        test___168.soft_fail_to_hard()
    }
    use super::*;
}
