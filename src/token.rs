const AMPERSAND: char = '&';
const ASTERISK: char = '*';
const BACKTICK: char = '`';
const BRACKET_CLOSE: char = ']';
const BRACKET_OPEN: char = '[';
const CIRCUMFLEX: char = '^';
const COMMA: char = ',';
const CURLY_BRACKET_CLOSE: char = '}';
const CURLY_BRACKET_OPEN: char = '{';
const DELIMITER: char = ';';
const EQUAL: char = '=';
const FULL_STOP: char = '.';
const GREATER_THAN: char = '>';
const HYPHEN: char = '-';
const LESS_THAN: char = '<';
const NEW_LINE: char = '\n';
const PAREN_CLOSE: char = ')';
const PAREN_OPEN: char = '(';
const PERCENT: char = '%';
const PLUS: char = '+';
const QUOTE_DOUBLE: char = '"';
const QUOTE_SINGLE: char = '\'';
const SLASH_FORWARD: char = '/';
const VERTICAL_BAR: char = '|';

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub value: String,
    pub category: Option<TokenCategory>,
    pub behavior: Vec<TokenBehavior>,
}

impl Token {
    fn new() -> Token {
        Token {
            value: String::new(),
            category: None,
            behavior: vec![],
        }
    }

    pub fn new_space(space: String) -> Token {
        Token {
            value: space,
            category: Some(TokenCategory::Space),
            behavior: vec![],
        }
    }

    pub fn newline() -> Token {
        Token {
            value: "\n".to_string(),
            category: Some(TokenCategory::NewLine),
            behavior: vec![],
        }
    }

    fn len(&self) -> usize {
        self.value.len()
    }

    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    fn count(&self, find: char) -> usize {
        self.value.match_indices(find).count()
    }

    fn setup(&mut self) {
        self.category = self.get_category();
        self.set_behavior();
    }

    fn get_category(&self) -> Option<TokenCategory> {
        if self.category.is_some() {
            return self.category.clone();
        }

        if self.value.len() == 1 {
            return match self.value.chars().nth(0).unwrap() {
                DELIMITER => Some(TokenCategory::Delimiter),
                NEW_LINE => Some(TokenCategory::NewLine),
                COMMA => Some(TokenCategory::Comma),
                PAREN_OPEN => Some(TokenCategory::ParenOpen),
                PAREN_CLOSE => Some(TokenCategory::ParenClose),
                AMPERSAND => Some(TokenCategory::Bitwise),
                VERTICAL_BAR => Some(TokenCategory::Bitwise),
                CIRCUMFLEX => Some(TokenCategory::Bitwise),
                LESS_THAN => Some(TokenCategory::Compare),
                PLUS => Some(TokenCategory::Operator),
                HYPHEN => Some(TokenCategory::Operator),
                ASTERISK => Some(TokenCategory::Operator),
                SLASH_FORWARD => Some(TokenCategory::Operator),
                PERCENT => Some(TokenCategory::Operator),
                _ => None,
            };
        }

        return match self.value.to_uppercase().as_str() {
            // Keywords
            "ABORT" => Some(TokenCategory::Keyword),
            "ABORTSESSION" => Some(TokenCategory::Keyword),
            "ABSENT" => Some(TokenCategory::Keyword),
            "ABSOLUTE" => Some(TokenCategory::Keyword),
            "ACCESS" => Some(TokenCategory::Keyword),
            "ACCESSIBLE" => Some(TokenCategory::Keyword),
            "ACCESS_LOCK" => Some(TokenCategory::Keyword),
            "ACCOUNT" => Some(TokenCategory::Keyword),
            "ACOSH" => Some(TokenCategory::Keyword),
            "ACTION" => Some(TokenCategory::Keyword),
            "ADA" => Some(TokenCategory::Keyword),
            "ADD" => Some(TokenCategory::Keyword),
            "ADD_MONTHS" => Some(TokenCategory::Keyword),
            "ADMIN" => Some(TokenCategory::Keyword),
            "AFTER" => Some(TokenCategory::Keyword),
            "AGGREGATE" => Some(TokenCategory::Keyword),
            "ALIAS" => Some(TokenCategory::Keyword),
            "ALL" => Some(TokenCategory::Keyword),
            "ALLOCATE" => Some(TokenCategory::Keyword),
            "ALLOW" => Some(TokenCategory::Keyword),
            "ALTER" => Some(TokenCategory::Keyword),
            "ALTERAND" => Some(TokenCategory::Keyword),
            "AMP" => Some(TokenCategory::Keyword),
            "ANALYSE" => Some(TokenCategory::Keyword),
            "ANALYZE" => Some(TokenCategory::Keyword),
            "AND" => Some(TokenCategory::Keyword),
            "ANSIDATE" => Some(TokenCategory::Keyword),
            "ANY" => Some(TokenCategory::Keyword),
            "ANY_VALUE" => Some(TokenCategory::Keyword),
            "ARE" => Some(TokenCategory::Keyword),
            "ARRAY" => Some(TokenCategory::Keyword),
            "ARRAY_AGG" => Some(TokenCategory::Keyword),
            "ARRAY_EXISTS" => Some(TokenCategory::Keyword),
            "ARRAY_MAX_CARDINALITY" => Some(TokenCategory::Keyword),
            "AS" => Some(TokenCategory::Keyword),
            "ASC" => Some(TokenCategory::Keyword),
            "ASCII" => Some(TokenCategory::Keyword),
            "ASENSITIVE" => Some(TokenCategory::Keyword),
            "ASINH" => Some(TokenCategory::Keyword),
            "ASSERTION" => Some(TokenCategory::Keyword),
            "ASSOCIATE" => Some(TokenCategory::Keyword),
            "ASUTIME" => Some(TokenCategory::Keyword),
            "ASYMMETRIC" => Some(TokenCategory::Keyword),
            "AT" => Some(TokenCategory::Keyword),
            "ATANH" => Some(TokenCategory::Keyword),
            "ATN2" => Some(TokenCategory::Keyword),
            "ATOMIC" => Some(TokenCategory::Keyword),
            "AUDIT" => Some(TokenCategory::Keyword),
            "AUTHORIZATION" => Some(TokenCategory::Keyword),
            "AUX" => Some(TokenCategory::Keyword),
            "AUXILIARY" => Some(TokenCategory::Keyword),
            "AVE" => Some(TokenCategory::Keyword),
            "AVERAGE" => Some(TokenCategory::Keyword),
            "BACKUP" => Some(TokenCategory::Keyword),
            "BEFORE" => Some(TokenCategory::Keyword),
            "BEGIN" => Some(TokenCategory::Keyword),
            "BEGIN_FRAME" => Some(TokenCategory::Keyword),
            "BEGIN_PARTITION" => Some(TokenCategory::Keyword),
            "BETWEEN" => Some(TokenCategory::Keyword),
            "BIT_LENGTH" => Some(TokenCategory::Keyword),
            "BOTH" => Some(TokenCategory::Keyword),
            "BREADTH" => Some(TokenCategory::Keyword),
            "BREAK" => Some(TokenCategory::Keyword),
            "BROWSE" => Some(TokenCategory::Keyword),
            "BT" => Some(TokenCategory::Keyword),
            "BTRIM" => Some(TokenCategory::Keyword),
            "BUFFERPOOL" => Some(TokenCategory::Keyword),
            "BULK" => Some(TokenCategory::Keyword),
            "BUT" => Some(TokenCategory::Keyword),
            "BY" => Some(TokenCategory::Keyword),
            "BYTE" => Some(TokenCategory::Keyword),
            "BYTEINT" => Some(TokenCategory::Keyword),
            "BYTES" => Some(TokenCategory::Keyword),
            "CALLED" => Some(TokenCategory::Keyword),
            "CAPTURE" => Some(TokenCategory::Keyword),
            "CARDINALITY" => Some(TokenCategory::Keyword),
            "CASCADE" => Some(TokenCategory::Keyword),
            "CASCADED" => Some(TokenCategory::Keyword),
            "CASE" => Some(TokenCategory::Keyword),
            "CASESPECIFIC" => Some(TokenCategory::Keyword),
            "CASE_N" => Some(TokenCategory::Keyword),
            "CATALOG" => Some(TokenCategory::Keyword),
            "CCSID" => Some(TokenCategory::Keyword),
            "CD" => Some(TokenCategory::Keyword),
            "CHANGE" => Some(TokenCategory::Keyword),
            "CHAR2HEXINT" => Some(TokenCategory::Keyword),
            "CHARACTER" => Some(TokenCategory::Keyword),
            "CHARACTERS" => Some(TokenCategory::Keyword),
            "CHARINDEX" => Some(TokenCategory::Keyword),
            "CHARS" => Some(TokenCategory::Keyword),
            "CHECK" => Some(TokenCategory::Keyword),
            "CHECKPOINT" => Some(TokenCategory::Keyword),
            "CLASS" => Some(TokenCategory::Keyword),
            "CLASSIFIER" => Some(TokenCategory::Keyword),
            "CLOB" => Some(TokenCategory::Keyword),
            "CLONE" => Some(TokenCategory::Keyword),
            "CLOSE" => Some(TokenCategory::Keyword),
            "CLUSTER" => Some(TokenCategory::Keyword),
            "CLUSTERED" => Some(TokenCategory::Keyword),
            "CM" => Some(TokenCategory::Keyword),
            "COLLATE" => Some(TokenCategory::Keyword),
            "COLLATION" => Some(TokenCategory::Keyword),
            "COLLECT" => Some(TokenCategory::Keyword),
            "COLLECTION" => Some(TokenCategory::Keyword),
            "COLLID" => Some(TokenCategory::Keyword),
            "COLUMN" => Some(TokenCategory::Keyword),
            "COLUMN_VALUE" => Some(TokenCategory::Keyword),
            "COMMENT" => Some(TokenCategory::Keyword),
            "COMMIT" => Some(TokenCategory::Keyword),
            "COMPLETION" => Some(TokenCategory::Keyword),
            "COMPRESS" => Some(TokenCategory::Keyword),
            "COMPUTE" => Some(TokenCategory::Keyword),
            "CONCURRENTLY" => Some(TokenCategory::Keyword),
            "CONDITION" => Some(TokenCategory::Keyword),
            "CONNECT" => Some(TokenCategory::Keyword),
            "CONNECTION" => Some(TokenCategory::Keyword),
            "CONSTRAINT" => Some(TokenCategory::Keyword),
            "CONSTRAINTS" => Some(TokenCategory::Keyword),
            "CONSTRUCTOR" => Some(TokenCategory::Keyword),
            "CONTAINS" => Some(TokenCategory::Keyword),
            "CONTAINSTABLE" => Some(TokenCategory::Keyword),
            "CONTENT" => Some(TokenCategory::Keyword),
            "CONTINUE" => Some(TokenCategory::Keyword),
            "CONVERT_TABLE_HEADER" => Some(TokenCategory::Keyword),
            "COPY" => Some(TokenCategory::Keyword),
            "CORR" => Some(TokenCategory::Keyword),
            "CORRESPONDING" => Some(TokenCategory::Keyword),
            "COSH" => Some(TokenCategory::Keyword),
            "COVAR_POP" => Some(TokenCategory::Keyword),
            "COVAR_SAMP" => Some(TokenCategory::Keyword),
            "CREATE" => Some(TokenCategory::Keyword),
            "CROSS" => Some(TokenCategory::Keyword),
            "CS" => Some(TokenCategory::Keyword),
            "CSUM" => Some(TokenCategory::Keyword),
            "CT" => Some(TokenCategory::Keyword),
            "CUBE" => Some(TokenCategory::Keyword),
            "CUME_DIST" => Some(TokenCategory::Keyword),
            "CURRENT" => Some(TokenCategory::Keyword),
            "CURRENT_CATALOG" => Some(TokenCategory::Keyword),
            "CURRENT_DEFAULT_TRANSFORM_GROUP" => Some(TokenCategory::Keyword),
            "CURRENT_LC_CTYPE" => Some(TokenCategory::Keyword),
            "CURRENT_PATH" => Some(TokenCategory::Keyword),
            "CURRENT_ROLE" => Some(TokenCategory::Keyword),
            "CURRENT_ROW" => Some(TokenCategory::Keyword),
            "CURRENT_SCHEMA" => Some(TokenCategory::Keyword),
            "CURRENT_SERVER" => Some(TokenCategory::Keyword),
            "CURRENT_TIMEZONE" => Some(TokenCategory::Keyword),
            "CURRENT_TRANSFORM_GROUP_FOR_TYPE" => Some(TokenCategory::Keyword),
            "CURRVAL" => Some(TokenCategory::Keyword),
            "CURSOR" => Some(TokenCategory::Keyword),
            "CV" => Some(TokenCategory::Keyword),
            "CYCLE" => Some(TokenCategory::Keyword),
            "DATA" => Some(TokenCategory::Keyword),
            "DATABASE" => Some(TokenCategory::Keyword),
            "DATABASES" => Some(TokenCategory::Keyword),
            "DATABLOCKSIZE" => Some(TokenCategory::Keyword),
            "DATALENGTH" => Some(TokenCategory::Keyword),
            "DATEADD" => Some(TokenCategory::Keyword),
            "DATEFORM" => Some(TokenCategory::Keyword),
            "DATEFROMPARTS" => Some(TokenCategory::Keyword),
            "DATENAME" => Some(TokenCategory::Keyword),
            "DATEPART" => Some(TokenCategory::Keyword),
            "DAYS" => Some(TokenCategory::Keyword),
            "DAY_HOUR" => Some(TokenCategory::Keyword),
            "DAY_MICROSECOND" => Some(TokenCategory::Keyword),
            "DAY_MINUTE" => Some(TokenCategory::Keyword),
            "DAY_SECOND" => Some(TokenCategory::Keyword),
            "DBCC" => Some(TokenCategory::Keyword),
            "DBINFO" => Some(TokenCategory::Keyword),
            "DEALLOCATE" => Some(TokenCategory::Keyword),
            "DECFLOAT" => Some(TokenCategory::Keyword),
            "DECLARE" => Some(TokenCategory::Keyword),
            "DEFAULT" => Some(TokenCategory::Keyword),
            "DEFERRABLE" => Some(TokenCategory::Keyword),
            "DEFERRED" => Some(TokenCategory::Keyword),
            "DEFINE" => Some(TokenCategory::Keyword),
            "DEL" => Some(TokenCategory::Keyword),
            "DELAYED" => Some(TokenCategory::Keyword),
            "DELETE" => Some(TokenCategory::Keyword),
            "DENSE_RANK" => Some(TokenCategory::Keyword),
            "DENY" => Some(TokenCategory::Keyword),
            "DEPTH" => Some(TokenCategory::Keyword),
            "DEREF" => Some(TokenCategory::Keyword),
            "DESC" => Some(TokenCategory::Keyword),
            "DESCRIBE" => Some(TokenCategory::Keyword),
            "DESCRIPTOR" => Some(TokenCategory::Keyword),
            "DESTROY" => Some(TokenCategory::Keyword),
            "DESTRUCTOR" => Some(TokenCategory::Keyword),
            "DETERMINISTIC" => Some(TokenCategory::Keyword),
            "DIAGNOSTIC" => Some(TokenCategory::Keyword),
            "DIAGNOSTICS" => Some(TokenCategory::Keyword),
            "DICTIONARY" => Some(TokenCategory::Keyword),
            "DIFFERENCE" => Some(TokenCategory::Keyword),
            "DISABLE" => Some(TokenCategory::Keyword),
            "DISABLED" => Some(TokenCategory::Keyword),
            "DISALLOW" => Some(TokenCategory::Keyword),
            "DISCONNECT" => Some(TokenCategory::Keyword),
            "DISK" => Some(TokenCategory::Keyword),
            "DISTINCT" => Some(TokenCategory::Keyword),
            "DISTINCTROW" => Some(TokenCategory::Keyword),
            "DISTRIBUTED" => Some(TokenCategory::Keyword),
            "DO" => Some(TokenCategory::Keyword),
            "DOCUMENT" => Some(TokenCategory::Keyword),
            "DOMAIN" => Some(TokenCategory::Keyword),
            "DROP" => Some(TokenCategory::Keyword),
            "DSSIZE" => Some(TokenCategory::Keyword),
            "DUAL" => Some(TokenCategory::Keyword),
            "DUMP" => Some(TokenCategory::Keyword),
            "DYNAMIC" => Some(TokenCategory::Keyword),
            "EACH" => Some(TokenCategory::Keyword),
            "ECHO" => Some(TokenCategory::Keyword),
            "EDITPROC" => Some(TokenCategory::Keyword),
            "ELEMENT" => Some(TokenCategory::Keyword),
            "ELSE" => Some(TokenCategory::Keyword),
            "ELSEIF" => Some(TokenCategory::Keyword),
            "EMPTY" => Some(TokenCategory::Keyword),
            "ENABLED" => Some(TokenCategory::Keyword),
            "ENCLOSED" => Some(TokenCategory::Keyword),
            "ENCODING" => Some(TokenCategory::Keyword),
            "ENCRYPTION" => Some(TokenCategory::Keyword),
            "END" => Some(TokenCategory::Keyword),
            "END-EXEC" => Some(TokenCategory::Keyword),
            "ENDING" => Some(TokenCategory::Keyword),
            "END_FRAME" => Some(TokenCategory::Keyword),
            "END_PARTITION" => Some(TokenCategory::Keyword),
            "EQ" => Some(TokenCategory::Keyword),
            "EQUALS" => Some(TokenCategory::Keyword),
            "ERASE" => Some(TokenCategory::Keyword),
            "ERRLVL" => Some(TokenCategory::Keyword),
            "ERROR" => Some(TokenCategory::Keyword),
            "ERRORFILES" => Some(TokenCategory::Keyword),
            "ERRORTABLES" => Some(TokenCategory::Keyword),
            "ESCAPE" => Some(TokenCategory::Keyword),
            "ESCAPED" => Some(TokenCategory::Keyword),
            "ET" => Some(TokenCategory::Keyword),
            "EVERY" => Some(TokenCategory::Keyword),
            "EXCEPT" => Some(TokenCategory::Keyword),
            "EXCEPTION" => Some(TokenCategory::Keyword),
            "EXCLUSIVE" => Some(TokenCategory::Keyword),
            "EXISTS" => Some(TokenCategory::Keyword),
            "EXIT" => Some(TokenCategory::Keyword),
            "EXPLAIN" => Some(TokenCategory::Keyword),
            "EXTERNAL" => Some(TokenCategory::Keyword),
            "FALLBACK" => Some(TokenCategory::Keyword),
            "FALSE" => Some(TokenCategory::Keyword),
            "FASTEXPORT" => Some(TokenCategory::Keyword),
            "FENCED" => Some(TokenCategory::Keyword),
            "FETCH" => Some(TokenCategory::Keyword),
            "FIELD" => Some(TokenCategory::Keyword),
            "FIELDPROC" => Some(TokenCategory::Keyword),
            "FILE" => Some(TokenCategory::Keyword),
            "FILLFACTOR" => Some(TokenCategory::Keyword),
            "FILTER" => Some(TokenCategory::Keyword),
            "FINAL" => Some(TokenCategory::Keyword),
            "FLOAT4" => Some(TokenCategory::Keyword),
            "FLOAT8" => Some(TokenCategory::Keyword),
            "FOR" => Some(TokenCategory::Keyword),
            "FORCE" => Some(TokenCategory::Keyword),
            "FOREIGN" => Some(TokenCategory::Keyword),
            "FORMAT" => Some(TokenCategory::Keyword),
            "FORTRAN" => Some(TokenCategory::Keyword),
            "FOUND" => Some(TokenCategory::Keyword),
            "FRAME_ROW" => Some(TokenCategory::Keyword),
            "FREE" => Some(TokenCategory::Keyword),
            "FREESPACE" => Some(TokenCategory::Keyword),
            "FREETEXT" => Some(TokenCategory::Keyword),
            "FREETEXTTABLE" => Some(TokenCategory::Keyword),
            "FREEZE" => Some(TokenCategory::Keyword),
            "FROM" => Some(TokenCategory::Keyword),
            "FULL" => Some(TokenCategory::Keyword),
            "FULLTEXT" => Some(TokenCategory::Keyword),
            "FUNCTION" => Some(TokenCategory::Keyword),
            "FUSION" => Some(TokenCategory::Keyword),
            "GE" => Some(TokenCategory::Keyword),
            "GENERAL" => Some(TokenCategory::Keyword),
            "GENERATED" => Some(TokenCategory::Keyword),
            "GET" => Some(TokenCategory::Keyword),
            "GETUTCDATE" => Some(TokenCategory::Keyword),
            "GIVE" => Some(TokenCategory::Keyword),
            "GLOBAL" => Some(TokenCategory::Keyword),
            "GO" => Some(TokenCategory::Keyword),
            "GOTO" => Some(TokenCategory::Keyword),
            "GRANT" => Some(TokenCategory::Keyword),
            "GRAPHIC" => Some(TokenCategory::Keyword),
            "GROUP" => Some(TokenCategory::Keyword),
            "GROUPING" => Some(TokenCategory::Keyword),
            "GROUPS" => Some(TokenCategory::Keyword),
            "GT" => Some(TokenCategory::Keyword),
            "HANDLER" => Some(TokenCategory::Keyword),
            "HASH" => Some(TokenCategory::Keyword),
            "HASHAMP" => Some(TokenCategory::Keyword),
            "HASHBAKAMP" => Some(TokenCategory::Keyword),
            "HASHBUCKET" => Some(TokenCategory::Keyword),
            "HASHROW" => Some(TokenCategory::Keyword),
            "HAVING" => Some(TokenCategory::Keyword),
            "HELP" => Some(TokenCategory::Keyword),
            "HIGH_PRIORITY" => Some(TokenCategory::Keyword),
            "HOLD" => Some(TokenCategory::Keyword),
            "HOLDLOCK" => Some(TokenCategory::Keyword),
            "HOST" => Some(TokenCategory::Keyword),
            "HOURS" => Some(TokenCategory::Keyword),
            "HOUR_MICROSECOND" => Some(TokenCategory::Keyword),
            "HOUR_MINUTE" => Some(TokenCategory::Keyword),
            "HOUR_SECOND" => Some(TokenCategory::Keyword),
            "IDENTIFIED" => Some(TokenCategory::Keyword),
            "IDENTITY" => Some(TokenCategory::Keyword),
            "IDENTITYCOL" => Some(TokenCategory::Keyword),
            "IDENTITY_INSERT" => Some(TokenCategory::Keyword),
            "IGNORE" => Some(TokenCategory::Keyword),
            "ILIKE" => Some(TokenCategory::Keyword),
            "IMMEDIATE" => Some(TokenCategory::Keyword),
            "IN" => Some(TokenCategory::Keyword),
            "INCLUDE" => Some(TokenCategory::Keyword),
            "INCLUSIVE" => Some(TokenCategory::Keyword),
            "INCONSISTENT" => Some(TokenCategory::Keyword),
            "INCREMENT" => Some(TokenCategory::Keyword),
            "INDEX" => Some(TokenCategory::Keyword),
            "INDICATOR" => Some(TokenCategory::Keyword),
            "INFILE" => Some(TokenCategory::Keyword),
            "INHERIT" => Some(TokenCategory::Keyword),
            "INITIAL" => Some(TokenCategory::Keyword),
            "INITIALIZE" => Some(TokenCategory::Keyword),
            "INITIALLY" => Some(TokenCategory::Keyword),
            "INITIATE" => Some(TokenCategory::Keyword),
            "INNER" => Some(TokenCategory::Keyword),
            "INOUT" => Some(TokenCategory::Keyword),
            "INPUT" => Some(TokenCategory::Keyword),
            "INS" => Some(TokenCategory::Keyword),
            "INSENSITIVE" => Some(TokenCategory::Keyword),
            "INSERT" => Some(TokenCategory::Keyword),
            "INSTEAD" => Some(TokenCategory::Keyword),
            "INT1" => Some(TokenCategory::Keyword),
            "INT2" => Some(TokenCategory::Keyword),
            "INT3" => Some(TokenCategory::Keyword),
            "INT4" => Some(TokenCategory::Keyword),
            "INT8" => Some(TokenCategory::Keyword),
            "INTEGERDATE" => Some(TokenCategory::Keyword),
            "INTERSECT" => Some(TokenCategory::Keyword),
            "INTERSECTION" => Some(TokenCategory::Keyword),
            "INTERVAL" => Some(TokenCategory::Keyword),
            "INTO" => Some(TokenCategory::Keyword),
            "IO_AFTER_GTIDS" => Some(TokenCategory::Keyword),
            "IO_BEFORE_GTIDS" => Some(TokenCategory::Keyword),
            "IS" => Some(TokenCategory::Keyword),
            "ISDATE" => Some(TokenCategory::Keyword),
            "ISNUMERIC" => Some(TokenCategory::Keyword),
            "ISOBID" => Some(TokenCategory::Keyword),
            "ISOLATION" => Some(TokenCategory::Keyword),
            "ITERATE" => Some(TokenCategory::Keyword),
            "JAR" => Some(TokenCategory::Keyword),
            "JOIN" => Some(TokenCategory::Keyword),
            "JOURNAL" => Some(TokenCategory::Keyword),
            "JSON" => Some(TokenCategory::Keyword),
            "JSON_ARRAY" => Some(TokenCategory::Keyword),
            "JSON_ARRAYAGG" => Some(TokenCategory::Keyword),
            "JSON_EXISTS" => Some(TokenCategory::Keyword),
            "JSON_OBJECT" => Some(TokenCategory::Keyword),
            "JSON_OBJECTAGG" => Some(TokenCategory::Keyword),
            "JSON_QUERY" => Some(TokenCategory::Keyword),
            "JSON_SCALAR" => Some(TokenCategory::Keyword),
            "JSON_SERIALIZE" => Some(TokenCategory::Keyword),
            "JSON_TABLE" => Some(TokenCategory::Keyword),
            "JSON_TABLE_PRIMITIVE" => Some(TokenCategory::Keyword),
            "JSON_VALUE" => Some(TokenCategory::Keyword),
            "KEEP" => Some(TokenCategory::Keyword),
            "KEYS" => Some(TokenCategory::Keyword),
            "KILL" => Some(TokenCategory::Keyword),
            "KURTOSIS" => Some(TokenCategory::Keyword),
            "LABEL" => Some(TokenCategory::Keyword),
            "LANGUAGE" => Some(TokenCategory::Keyword),
            "LARGE" => Some(TokenCategory::Keyword),
            "LATERAL" => Some(TokenCategory::Keyword),
            "LC_CTYPE" => Some(TokenCategory::Keyword),
            "LE" => Some(TokenCategory::Keyword),
            "LEADING" => Some(TokenCategory::Keyword),
            "LEAVE" => Some(TokenCategory::Keyword),
            "LEN" => Some(TokenCategory::Keyword),
            "LESS" => Some(TokenCategory::Keyword),
            "LEVEL" => Some(TokenCategory::Keyword),
            "LIKE" => Some(TokenCategory::Keyword),
            "LIKE_REGEX" => Some(TokenCategory::Keyword),
            "LIMIT" => Some(TokenCategory::Keyword),
            "LINEAR" => Some(TokenCategory::Keyword),
            "LINENO" => Some(TokenCategory::Keyword),
            "LINES" => Some(TokenCategory::Keyword),
            "LISTAGG" => Some(TokenCategory::Keyword),
            "LOAD" => Some(TokenCategory::Keyword),
            "LOADING" => Some(TokenCategory::Keyword),
            "LOCAL" => Some(TokenCategory::Keyword),
            "LOCALE" => Some(TokenCategory::Keyword),
            "LOCATOR" => Some(TokenCategory::Keyword),
            "LOCATORS" => Some(TokenCategory::Keyword),
            "LOCK" => Some(TokenCategory::Keyword),
            "LOCKING" => Some(TokenCategory::Keyword),
            "LOCKMAX" => Some(TokenCategory::Keyword),
            "LOCKSIZE" => Some(TokenCategory::Keyword),
            "LOGGING" => Some(TokenCategory::Keyword),
            "LOGON" => Some(TokenCategory::Keyword),
            "LONG" => Some(TokenCategory::Keyword),
            "LOOP" => Some(TokenCategory::Keyword),
            "LOW_PRIORITY" => Some(TokenCategory::Keyword),
            "LT" => Some(TokenCategory::Keyword),
            "MACRO" => Some(TokenCategory::Keyword),
            "MAINTAINED" => Some(TokenCategory::Keyword),
            "MAP" => Some(TokenCategory::Keyword),
            "MASTER_BIND" => Some(TokenCategory::Keyword),
            "MASTER_SSL_VERIFY_SERVER_CERT" => Some(TokenCategory::Keyword),
            "MATCH" => Some(TokenCategory::Keyword),
            "MATCHES" => Some(TokenCategory::Keyword),
            "MATCH_NUMBER" => Some(TokenCategory::Keyword),
            "MATCH_RECOGNIZE" => Some(TokenCategory::Keyword),
            "MATERIALIZED" => Some(TokenCategory::Keyword),
            "MAVG" => Some(TokenCategory::Keyword),
            "MAXEXTENTS" => Some(TokenCategory::Keyword),
            "MAXIMUM" => Some(TokenCategory::Keyword),
            "MAXVALUE" => Some(TokenCategory::Keyword),
            "MCHARACTERS" => Some(TokenCategory::Keyword),
            "MDIFF" => Some(TokenCategory::Keyword),
            "MEMBER" => Some(TokenCategory::Keyword),
            "MERGE" => Some(TokenCategory::Keyword),
            "METHOD" => Some(TokenCategory::Keyword),
            "MICROSECONDS" => Some(TokenCategory::Keyword),
            "MIDDLEINT" => Some(TokenCategory::Keyword),
            "MINDEX" => Some(TokenCategory::Keyword),
            "MINIMUM" => Some(TokenCategory::Keyword),
            "MINUS" => Some(TokenCategory::Keyword),
            "MINUTES" => Some(TokenCategory::Keyword),
            "MINUTE_MICROSECOND" => Some(TokenCategory::Keyword),
            "MINUTE_SECOND" => Some(TokenCategory::Keyword),
            "MLINREG" => Some(TokenCategory::Keyword),
            "MLOAD" => Some(TokenCategory::Keyword),
            "MLSLABEL" => Some(TokenCategory::Keyword),
            "MODE" => Some(TokenCategory::Keyword),
            "MODIFIES" => Some(TokenCategory::Keyword),
            "MODIFY" => Some(TokenCategory::Keyword),
            "MODULE" => Some(TokenCategory::Keyword),
            "MONRESOURCE" => Some(TokenCategory::Keyword),
            "MONSESSION" => Some(TokenCategory::Keyword),
            "MONTHS" => Some(TokenCategory::Keyword),
            "MSUBSTR" => Some(TokenCategory::Keyword),
            "MSUM" => Some(TokenCategory::Keyword),
            "MULTISET" => Some(TokenCategory::Keyword),
            "NAMED" => Some(TokenCategory::Keyword),
            "NAMES" => Some(TokenCategory::Keyword),
            "NATIONAL" => Some(TokenCategory::Keyword),
            "NATURAL" => Some(TokenCategory::Keyword),
            "NCLOB" => Some(TokenCategory::Keyword),
            "NE" => Some(TokenCategory::Keyword),
            "NESTED_TABLE_ID" => Some(TokenCategory::Keyword),
            "NEW" => Some(TokenCategory::Keyword),
            "NEW_TABLE" => Some(TokenCategory::Keyword),
            "NEXT" => Some(TokenCategory::Keyword),
            "NEXTVAL" => Some(TokenCategory::Keyword),
            "NO" => Some(TokenCategory::Keyword),
            "NOAUDIT" => Some(TokenCategory::Keyword),
            "NOCHECK" => Some(TokenCategory::Keyword),
            "NOCOMPRESS" => Some(TokenCategory::Keyword),
            "NONCLUSTERED" => Some(TokenCategory::Keyword),
            "NONE" => Some(TokenCategory::Keyword),
            "NORMALIZE" => Some(TokenCategory::Keyword),
            "NOT" => Some(TokenCategory::Keyword),
            "NOTNULL" => Some(TokenCategory::Keyword),
            "NOWAIT" => Some(TokenCategory::Keyword),
            "NO_WRITE_TO_BINLOG" => Some(TokenCategory::Keyword),
            "NTH_VALUE" => Some(TokenCategory::Keyword),
            "NTILE" => Some(TokenCategory::Keyword),
            "NULL" => Some(TokenCategory::Keyword),
            "NULLIFZERO" => Some(TokenCategory::Keyword),
            "NULLS" => Some(TokenCategory::Keyword),
            "NUMBER" => Some(TokenCategory::Keyword),
            "NUMPARTS" => Some(TokenCategory::Keyword),
            "OBID" => Some(TokenCategory::Keyword),
            "OBJECT" => Some(TokenCategory::Keyword),
            "OBJECTS" => Some(TokenCategory::Keyword),
            "OCCURRENCES_REGEX" => Some(TokenCategory::Keyword),
            "OCTET_LENGTH" => Some(TokenCategory::Keyword),
            "OF" => Some(TokenCategory::Keyword),
            "OFF" => Some(TokenCategory::Keyword),
            "OFFLINE" => Some(TokenCategory::Keyword),
            "OFFSET" => Some(TokenCategory::Keyword),
            "OFFSETS" => Some(TokenCategory::Keyword),
            "OLD" => Some(TokenCategory::Keyword),
            "OLD_TABLE" => Some(TokenCategory::Keyword),
            "OMIT" => Some(TokenCategory::Keyword),
            "ON" => Some(TokenCategory::Keyword),
            "ONE" => Some(TokenCategory::Keyword),
            "ONLINE" => Some(TokenCategory::Keyword),
            "ONLY" => Some(TokenCategory::Keyword),
            "OPEN" => Some(TokenCategory::Keyword),
            "OPENDATASOURCE" => Some(TokenCategory::Keyword),
            "OPENQUERY" => Some(TokenCategory::Keyword),
            "OPENROWSET" => Some(TokenCategory::Keyword),
            "OPENXML" => Some(TokenCategory::Keyword),
            "OPERATION" => Some(TokenCategory::Keyword),
            "OPTIMIZATION" => Some(TokenCategory::Keyword),
            "OPTIMIZE" => Some(TokenCategory::Keyword),
            "OPTIMIZER_COSTS" => Some(TokenCategory::Keyword),
            "OPTION" => Some(TokenCategory::Keyword),
            "OPTIONALLY" => Some(TokenCategory::Keyword),
            "OR" => Some(TokenCategory::Keyword),
            "ORDER" => Some(TokenCategory::Keyword),
            "ORDINALITY" => Some(TokenCategory::Keyword),
            "ORGANIZATION" => Some(TokenCategory::Keyword),
            "OUT" => Some(TokenCategory::Keyword),
            "OUTER" => Some(TokenCategory::Keyword),
            "OUTFILE" => Some(TokenCategory::Keyword),
            "OUTPUT" => Some(TokenCategory::Keyword),
            "OVER" => Some(TokenCategory::Keyword),
            "OVERLAPS" => Some(TokenCategory::Keyword),
            "OVERLAY" => Some(TokenCategory::Keyword),
            "OVERRIDE" => Some(TokenCategory::Keyword),
            "PACKAGE" => Some(TokenCategory::Keyword),
            "PAD" => Some(TokenCategory::Keyword),
            "PADDED" => Some(TokenCategory::Keyword),
            "PARAMETER" => Some(TokenCategory::Keyword),
            "PARAMETERS" => Some(TokenCategory::Keyword),
            "PART" => Some(TokenCategory::Keyword),
            "PARTIAL" => Some(TokenCategory::Keyword),
            "PARTITION" => Some(TokenCategory::Keyword),
            "PARTITIONED" => Some(TokenCategory::Keyword),
            "PARTITIONING" => Some(TokenCategory::Keyword),
            "PASCAL" => Some(TokenCategory::Keyword),
            "PASSWORD" => Some(TokenCategory::Keyword),
            "PATH" => Some(TokenCategory::Keyword),
            "PATINDEX" => Some(TokenCategory::Keyword),
            "PATTERN" => Some(TokenCategory::Keyword),
            "PCTFREE" => Some(TokenCategory::Keyword),
            "PER" => Some(TokenCategory::Keyword),
            "PERCENT" => Some(TokenCategory::Keyword),
            "PERCENTILE_CONT" => Some(TokenCategory::Keyword),
            "PERCENTILE_DISC" => Some(TokenCategory::Keyword),
            "PERCENT_RANK" => Some(TokenCategory::Keyword),
            "PERIOD" => Some(TokenCategory::Keyword),
            "PERM" => Some(TokenCategory::Keyword),
            "PERMANENT" => Some(TokenCategory::Keyword),
            "PIECESIZE" => Some(TokenCategory::Keyword),
            "PIVOT" => Some(TokenCategory::Keyword),
            "PLACING" => Some(TokenCategory::Keyword),
            "PLAN" => Some(TokenCategory::Keyword),
            "PORTION" => Some(TokenCategory::Keyword),
            "POSITION_REGEX" => Some(TokenCategory::Keyword),
            "POSTFIX" => Some(TokenCategory::Keyword),
            "PRECEDES" => Some(TokenCategory::Keyword),
            "PRECISION" => Some(TokenCategory::Keyword),
            "PREFIX" => Some(TokenCategory::Keyword),
            "PREORDER" => Some(TokenCategory::Keyword),
            "PREPARE" => Some(TokenCategory::Keyword),
            "PRESERVE" => Some(TokenCategory::Keyword),
            "PREVVAL" => Some(TokenCategory::Keyword),
            "PRIMARY" => Some(TokenCategory::Keyword),
            "PRINT" => Some(TokenCategory::Keyword),
            "PRIOR" => Some(TokenCategory::Keyword),
            "PRIQTY" => Some(TokenCategory::Keyword),
            "PRIVATE" => Some(TokenCategory::Keyword),
            "PRIVILEGES" => Some(TokenCategory::Keyword),
            "PROC" => Some(TokenCategory::Keyword),
            "PROCEDURE" => Some(TokenCategory::Keyword),
            "PROFILE" => Some(TokenCategory::Keyword),
            "PROGRAM" => Some(TokenCategory::Keyword),
            "PROPORTIONAL" => Some(TokenCategory::Keyword),
            "PROTECTION" => Some(TokenCategory::Keyword),
            "PSID" => Some(TokenCategory::Keyword),
            "PTF" => Some(TokenCategory::Keyword),
            "PUBLIC" => Some(TokenCategory::Keyword),
            "PURGE" => Some(TokenCategory::Keyword),
            "QUALIFIED" => Some(TokenCategory::Keyword),
            "QUALIFY" => Some(TokenCategory::Keyword),
            "QUANTILE" => Some(TokenCategory::Keyword),
            "QUERY" => Some(TokenCategory::Keyword),
            "QUERYNO" => Some(TokenCategory::Keyword),
            "QUOTENAME" => Some(TokenCategory::Keyword),
            "RAISERROR" => Some(TokenCategory::Keyword),
            "RANDOM" => Some(TokenCategory::Keyword),
            "RANGE" => Some(TokenCategory::Keyword),
            "RANGE_N" => Some(TokenCategory::Keyword),
            "RANK" => Some(TokenCategory::Keyword),
            "RAW" => Some(TokenCategory::Keyword),
            "READ" => Some(TokenCategory::Keyword),
            "READS" => Some(TokenCategory::Keyword),
            "READTEXT" => Some(TokenCategory::Keyword),
            "READ_WRITE" => Some(TokenCategory::Keyword),
            "RECONFIGURE" => Some(TokenCategory::Keyword),
            "RECURSIVE" => Some(TokenCategory::Keyword),
            "REF" => Some(TokenCategory::Keyword),
            "REFERENCES" => Some(TokenCategory::Keyword),
            "REFERENCING" => Some(TokenCategory::Keyword),
            "REFRESH" => Some(TokenCategory::Keyword),
            "REGEXP" => Some(TokenCategory::Keyword),
            "REGR_AVGX" => Some(TokenCategory::Keyword),
            "REGR_AVGY" => Some(TokenCategory::Keyword),
            "REGR_COUNT" => Some(TokenCategory::Keyword),
            "REGR_INTERCEPT" => Some(TokenCategory::Keyword),
            "REGR_R2" => Some(TokenCategory::Keyword),
            "REGR_SLOPE" => Some(TokenCategory::Keyword),
            "REGR_SXX" => Some(TokenCategory::Keyword),
            "REGR_SXY" => Some(TokenCategory::Keyword),
            "REGR_SYY" => Some(TokenCategory::Keyword),
            "RELATIVE" => Some(TokenCategory::Keyword),
            "RELEASE" => Some(TokenCategory::Keyword),
            "RENAME" => Some(TokenCategory::Keyword),
            "REPLICATE" => Some(TokenCategory::Keyword),
            "REPLICATION" => Some(TokenCategory::Keyword),
            "REPOVERRIDE" => Some(TokenCategory::Keyword),
            "REQUEST" => Some(TokenCategory::Keyword),
            "REQUIRE" => Some(TokenCategory::Keyword),
            "RESIGNAL" => Some(TokenCategory::Keyword),
            "RESOURCE" => Some(TokenCategory::Keyword),
            "RESTART" => Some(TokenCategory::Keyword),
            "RESTORE" => Some(TokenCategory::Keyword),
            "RESTRICT" => Some(TokenCategory::Keyword),
            "RESULT" => Some(TokenCategory::Keyword),
            "RESULT_SET_LOCATOR" => Some(TokenCategory::Keyword),
            "RESUME" => Some(TokenCategory::Keyword),
            "RET" => Some(TokenCategory::Keyword),
            "RETRIEVE" => Some(TokenCategory::Keyword),
            "RETURN" => Some(TokenCategory::Keyword),
            "RETURNING" => Some(TokenCategory::Keyword),
            "RETURNS" => Some(TokenCategory::Keyword),
            "REVALIDATE" => Some(TokenCategory::Keyword),
            "REVERT" => Some(TokenCategory::Keyword),
            "REVOKE" => Some(TokenCategory::Keyword),
            "RIGHTS" => Some(TokenCategory::Keyword),
            "RLIKE" => Some(TokenCategory::Keyword),
            "ROLE" => Some(TokenCategory::Keyword),
            "ROLLBACK" => Some(TokenCategory::Keyword),
            "ROLLFORWARD" => Some(TokenCategory::Keyword),
            "ROLLUP" => Some(TokenCategory::Keyword),
            "ROUND_CEILING" => Some(TokenCategory::Keyword),
            "ROUND_DOWN" => Some(TokenCategory::Keyword),
            "ROUND_FLOOR" => Some(TokenCategory::Keyword),
            "ROUND_HALF_DOWN" => Some(TokenCategory::Keyword),
            "ROUND_HALF_EVEN" => Some(TokenCategory::Keyword),
            "ROUND_HALF_UP" => Some(TokenCategory::Keyword),
            "ROUND_UP" => Some(TokenCategory::Keyword),
            "ROUTINE" => Some(TokenCategory::Keyword),
            "ROW" => Some(TokenCategory::Keyword),
            "ROWCOUNT" => Some(TokenCategory::Keyword),
            "ROWGUIDCOL" => Some(TokenCategory::Keyword),
            "ROWID" => Some(TokenCategory::Keyword),
            "ROWNUM" => Some(TokenCategory::Keyword),
            "ROWS" => Some(TokenCategory::Keyword),
            "ROWSET" => Some(TokenCategory::Keyword),
            "RULE" => Some(TokenCategory::Keyword),
            "RUN" => Some(TokenCategory::Keyword),
            "RUNNING" => Some(TokenCategory::Keyword),
            "SAMPLE" => Some(TokenCategory::Keyword),
            "SAMPLEID" => Some(TokenCategory::Keyword),
            "SAVE" => Some(TokenCategory::Keyword),
            "SAVEPOINT" => Some(TokenCategory::Keyword),
            "SCHEMA" => Some(TokenCategory::Keyword),
            "SCHEMAS" => Some(TokenCategory::Keyword),
            "SCOPE" => Some(TokenCategory::Keyword),
            "SCRATCHPAD" => Some(TokenCategory::Keyword),
            "SCROLL" => Some(TokenCategory::Keyword),
            "SEARCH" => Some(TokenCategory::Keyword),
            "SECONDS" => Some(TokenCategory::Keyword),
            "SECOND_MICROSECOND" => Some(TokenCategory::Keyword),
            "SECQTY" => Some(TokenCategory::Keyword),
            "SECTION" => Some(TokenCategory::Keyword),
            "SECURITY" => Some(TokenCategory::Keyword),
            "SECURITYAUDIT" => Some(TokenCategory::Keyword),
            "SEEK" => Some(TokenCategory::Keyword),
            "SEL" => Some(TokenCategory::Keyword),
            "SELECT" => Some(TokenCategory::Keyword),
            "SEMANTICKEYPHRASETABLE" => Some(TokenCategory::Keyword),
            "SEMANTICSIMILARITYDETAILSTABLE" => Some(TokenCategory::Keyword),
            "SEMANTICSIMILARITYTABLE" => Some(TokenCategory::Keyword),
            "SENSITIVE" => Some(TokenCategory::Keyword),
            "SEPARATOR" => Some(TokenCategory::Keyword),
            "SEQUENCE" => Some(TokenCategory::Keyword),
            "SESSION" => Some(TokenCategory::Keyword),
            "SESSIONPROPERTY" => Some(TokenCategory::Keyword),
            "SETRESRATE" => Some(TokenCategory::Keyword),
            "SETS" => Some(TokenCategory::Keyword),
            "SETSESSRATE" => Some(TokenCategory::Keyword),
            "SETUSER" => Some(TokenCategory::Keyword),
            "SHARE" => Some(TokenCategory::Keyword),
            "SHOW" => Some(TokenCategory::Keyword),
            "SHUTDOWN" => Some(TokenCategory::Keyword),
            "SIGNAL" => Some(TokenCategory::Keyword),
            "SIMILAR" => Some(TokenCategory::Keyword),
            "SIMPLE" => Some(TokenCategory::Keyword),
            "SINH" => Some(TokenCategory::Keyword),
            "SIZE" => Some(TokenCategory::Keyword),
            "SKEW" => Some(TokenCategory::Keyword),
            "SKIP" => Some(TokenCategory::Keyword),
            "SOME" => Some(TokenCategory::Keyword),
            "SOUNDEX" => Some(TokenCategory::Keyword),
            "SOURCE" => Some(TokenCategory::Keyword),
            "SPATIAL" => Some(TokenCategory::Keyword),
            "SPECIFIC" => Some(TokenCategory::Keyword),
            "SPECIFICTYPE" => Some(TokenCategory::Keyword),
            "SPOOL" => Some(TokenCategory::Keyword),
            "SQL" => Some(TokenCategory::Keyword),
            "SQLCA" => Some(TokenCategory::Keyword),
            "SQLCODE" => Some(TokenCategory::Keyword),
            "SQLERROR" => Some(TokenCategory::Keyword),
            "SQLEXCEPTION" => Some(TokenCategory::Keyword),
            "SQLSTATE" => Some(TokenCategory::Keyword),
            "SQLTEXT" => Some(TokenCategory::Keyword),
            "SQLWARNING" => Some(TokenCategory::Keyword),
            "SQL_BIG_RESULT" => Some(TokenCategory::Keyword),
            "SQL_CALC_FOUND_ROWS" => Some(TokenCategory::Keyword),
            "SQL_SMALL_RESULT" => Some(TokenCategory::Keyword),
            "SQUARE" => Some(TokenCategory::Keyword),
            "SS" => Some(TokenCategory::Keyword),
            "SSL" => Some(TokenCategory::Keyword),
            "STANDARD" => Some(TokenCategory::Keyword),
            "START" => Some(TokenCategory::Keyword),
            "STARTING" => Some(TokenCategory::Keyword),
            "STARTUP" => Some(TokenCategory::Keyword),
            "STATE" => Some(TokenCategory::Keyword),
            "STATEMENT" => Some(TokenCategory::Keyword),
            "STATIC" => Some(TokenCategory::Keyword),
            "STATISTICS" => Some(TokenCategory::Keyword),
            "STAY" => Some(TokenCategory::Keyword),
            "STDDEV_POP" => Some(TokenCategory::Keyword),
            "STDDEV_SAMP" => Some(TokenCategory::Keyword),
            "STEPINFO" => Some(TokenCategory::Keyword),
            "STOGROUP" => Some(TokenCategory::Keyword),
            "STORED" => Some(TokenCategory::Keyword),
            "STORES" => Some(TokenCategory::Keyword),
            "STR" => Some(TokenCategory::Keyword),
            "STRAIGHT_JOIN" => Some(TokenCategory::Keyword),
            "STRING_CS" => Some(TokenCategory::Keyword),
            "STRUCTURE" => Some(TokenCategory::Keyword),
            "STUFF" => Some(TokenCategory::Keyword),
            "STYLE" => Some(TokenCategory::Keyword),
            "SUBMULTISET" => Some(TokenCategory::Keyword),
            "SUBSCRIBER" => Some(TokenCategory::Keyword),
            "SUBSET" => Some(TokenCategory::Keyword),
            "SUBSTRING_REGEX" => Some(TokenCategory::Keyword),
            "SUCCEEDS" => Some(TokenCategory::Keyword),
            "SUCCESSFUL" => Some(TokenCategory::Keyword),
            "SUMMARY" => Some(TokenCategory::Keyword),
            "SUSPEND" => Some(TokenCategory::Keyword),
            "SYMMETRIC" => Some(TokenCategory::Keyword),
            "SYNONYM" => Some(TokenCategory::Keyword),
            "SYSDATETIME" => Some(TokenCategory::Keyword),
            "SYSTEM" => Some(TokenCategory::Keyword),
            "SYSTEM_TIME" => Some(TokenCategory::Keyword),
            "SYSTIMESTAMP" => Some(TokenCategory::Keyword),
            "TABLE" => Some(TokenCategory::Keyword),
            "TABLESAMPLE" => Some(TokenCategory::Keyword),
            "TABLESPACE" => Some(TokenCategory::Keyword),
            "TANH" => Some(TokenCategory::Keyword),
            "TBL_CS" => Some(TokenCategory::Keyword),
            "TEMPORARY" => Some(TokenCategory::Keyword),
            "TERMINATE" => Some(TokenCategory::Keyword),
            "TERMINATED" => Some(TokenCategory::Keyword),
            "TEXTSIZE" => Some(TokenCategory::Keyword),
            "THAN" => Some(TokenCategory::Keyword),
            "THEN" => Some(TokenCategory::Keyword),
            "THRESHOLD" => Some(TokenCategory::Keyword),
            "TIMEZONE_HOUR" => Some(TokenCategory::Keyword),
            "TIMEZONE_MINUTE" => Some(TokenCategory::Keyword),
            "TITLE" => Some(TokenCategory::Keyword),
            "TO" => Some(TokenCategory::Keyword),
            "TOP" => Some(TokenCategory::Keyword),
            "TRACE" => Some(TokenCategory::Keyword),
            "TRAILING" => Some(TokenCategory::Keyword),
            "TRAN" => Some(TokenCategory::Keyword),
            "TRANSACTION" => Some(TokenCategory::Keyword),
            "TRANSLATE" => Some(TokenCategory::Keyword),
            "TRANSLATE_CHK" => Some(TokenCategory::Keyword),
            "TRANSLATE_REGEX" => Some(TokenCategory::Keyword),
            "TRANSLATION" => Some(TokenCategory::Keyword),
            "TREAT" => Some(TokenCategory::Keyword),
            "TRIGGER" => Some(TokenCategory::Keyword),
            "TRIM_ARRAY" => Some(TokenCategory::Keyword),
            "TRUE" => Some(TokenCategory::Keyword),
            "TRY_CONVERT" => Some(TokenCategory::Keyword),
            "TSEQUAL" => Some(TokenCategory::Keyword),
            "TYPE" => Some(TokenCategory::Keyword),
            "UC" => Some(TokenCategory::Keyword),
            "UESCAPE" => Some(TokenCategory::Keyword),
            "UID" => Some(TokenCategory::Keyword),
            "UNDEFINED" => Some(TokenCategory::Keyword),
            "UNDER" => Some(TokenCategory::Keyword),
            "UNDO" => Some(TokenCategory::Keyword),
            "UNICODE" => Some(TokenCategory::Keyword),
            "UNION" => Some(TokenCategory::Keyword),
            "UNIQUE" => Some(TokenCategory::Keyword),
            "UNKNOWN" => Some(TokenCategory::Keyword),
            "UNLOCK" => Some(TokenCategory::Keyword),
            "UNNEST" => Some(TokenCategory::Keyword),
            "UNPIVOT" => Some(TokenCategory::Keyword),
            "UNSIGNED" => Some(TokenCategory::Keyword),
            "UNTIL" => Some(TokenCategory::Keyword),
            "UPD" => Some(TokenCategory::Keyword),
            "UPDATE" => Some(TokenCategory::Keyword),
            "UPDATETEXT" => Some(TokenCategory::Keyword),
            "UPPERCASE" => Some(TokenCategory::Keyword),
            "USAGE" => Some(TokenCategory::Keyword),
            "USE" => Some(TokenCategory::Keyword),
            "USER_NAME" => Some(TokenCategory::Keyword),
            "USING" => Some(TokenCategory::Keyword),
            "UTC_DATE" => Some(TokenCategory::Keyword),
            "UTC_TIME" => Some(TokenCategory::Keyword),
            "UTC_TIMESTAMP" => Some(TokenCategory::Keyword),
            "VALIDATE" => Some(TokenCategory::Keyword),
            "VALIDPROC" => Some(TokenCategory::Keyword),
            "VALUE" => Some(TokenCategory::Keyword),
            "VALUES" => Some(TokenCategory::Keyword),
            "VALUE_OF" => Some(TokenCategory::Keyword),
            "VARGRAPHIC" => Some(TokenCategory::Keyword),
            "VARIABLE" => Some(TokenCategory::Keyword),
            "VARIADIC" => Some(TokenCategory::Keyword),
            "VARIANT" => Some(TokenCategory::Keyword),
            "VARYING" => Some(TokenCategory::Keyword),
            "VAR_POP" => Some(TokenCategory::Keyword),
            "VAR_SAMP" => Some(TokenCategory::Keyword),
            "VCAT" => Some(TokenCategory::Keyword),
            "VERBOSE" => Some(TokenCategory::Keyword),
            "VERSIONING" => Some(TokenCategory::Keyword),
            "VIEW" => Some(TokenCategory::Keyword),
            "VIRTUAL" => Some(TokenCategory::Keyword),
            "VOLATILE" => Some(TokenCategory::Keyword),
            "VOLUMES" => Some(TokenCategory::Keyword),
            "WAIT" => Some(TokenCategory::Keyword),
            "WAITFOR" => Some(TokenCategory::Keyword),
            "WHEN" => Some(TokenCategory::Keyword),
            "WHENEVER" => Some(TokenCategory::Keyword),
            "WHERE" => Some(TokenCategory::Keyword),
            "WHILE" => Some(TokenCategory::Keyword),
            "WIDTH_BUCKET" => Some(TokenCategory::Keyword),
            "WINDOW" => Some(TokenCategory::Keyword),
            "WITH" => Some(TokenCategory::Keyword),
            "WITHIN" => Some(TokenCategory::Keyword),
            "WITHIN_GROUP" => Some(TokenCategory::Keyword),
            "WITHOUT" => Some(TokenCategory::Keyword),
            "WLM" => Some(TokenCategory::Keyword),
            "WORK" => Some(TokenCategory::Keyword),
            "WRITE" => Some(TokenCategory::Keyword),
            "WRITETEXT" => Some(TokenCategory::Keyword),
            "XMLCAST" => Some(TokenCategory::Keyword),
            "XMLEXISTS" => Some(TokenCategory::Keyword),
            "XMLNAMESPACES" => Some(TokenCategory::Keyword),
            "XOR" => Some(TokenCategory::Keyword),
            "YEARS" => Some(TokenCategory::Keyword),
            "YEAR_MONTH" => Some(TokenCategory::Keyword),
            "ZEROFILL" => Some(TokenCategory::Keyword),
            "ZEROIFNULL" => Some(TokenCategory::Keyword),
            "ZONE" => Some(TokenCategory::Keyword),

            // DataTypes
            "BIGINT" => Some(TokenCategory::DataType),
            "BINARY" => Some(TokenCategory::DataType),
            "BIT" => Some(TokenCategory::DataType),
            "BLOB" => Some(TokenCategory::DataType),
            "BOOL" => Some(TokenCategory::DataType),
            "BOOLEAN" => Some(TokenCategory::DataType),
            "CHAR" => Some(TokenCategory::DataType),
            "DATE" => Some(TokenCategory::DataType),
            "DATETIME" => Some(TokenCategory::DataType),
            "DATETIME2" => Some(TokenCategory::DataType),
            "DATETIMEOFFSET" => Some(TokenCategory::DataType),
            "DEC" => Some(TokenCategory::DataType),
            "DECIMAL" => Some(TokenCategory::DataType),
            "DOUBLE" => Some(TokenCategory::DataType),
            "ENUM" => Some(TokenCategory::DataType),
            "FLOAT" => Some(TokenCategory::DataType),
            "INT" => Some(TokenCategory::DataType),
            "INTEGER" => Some(TokenCategory::DataType),
            "KEY" => Some(TokenCategory::DataType),
            "LONGBLOB" => Some(TokenCategory::DataType),
            "LONGTEXT" => Some(TokenCategory::DataType),
            "MEDIUMBLOB" => Some(TokenCategory::DataType),
            "MEDIUMINT" => Some(TokenCategory::DataType),
            "MEDIUMTEXT" => Some(TokenCategory::DataType),
            "MONEY" => Some(TokenCategory::DataType),
            "NCHAR" => Some(TokenCategory::DataType),
            "NUMERIC" => Some(TokenCategory::DataType),
            "NVARCHAR" => Some(TokenCategory::DataType),
            "REAL" => Some(TokenCategory::DataType),
            "SET" => Some(TokenCategory::DataType),
            "SMALLDATETIME" => Some(TokenCategory::DataType),
            "SMALLINT" => Some(TokenCategory::DataType),
            "SMALLMONEY" => Some(TokenCategory::DataType),
            "SQL_VARIANT" => Some(TokenCategory::DataType),
            "TEXT" => Some(TokenCategory::DataType),
            "TIME" => Some(TokenCategory::DataType),
            "TIMESTAMP" => Some(TokenCategory::DataType),
            "TINYBLOB" => Some(TokenCategory::DataType),
            "TINYINT" => Some(TokenCategory::DataType),
            "TINYTEXT" => Some(TokenCategory::DataType),
            "UNIQUEIDENTIFIER" => Some(TokenCategory::DataType),
            "UUID" => Some(TokenCategory::DataType),
            "VARBINARY" => Some(TokenCategory::DataType),
            "VARBYTE" => Some(TokenCategory::DataType),
            "VARCHAR" => Some(TokenCategory::DataType),
            "VARCHAR2" => Some(TokenCategory::DataType),
            "VARCHARACTER" => Some(TokenCategory::DataType),
            "XML" => Some(TokenCategory::DataType),
            "YEAR" => Some(TokenCategory::DataType),

            // Methods
            "ABS" => Some(TokenCategory::Method),
            "ACOS" => Some(TokenCategory::Method),
            "ADDDATE" => Some(TokenCategory::Method),
            "ADDTIME" => Some(TokenCategory::Method),
            "ASIN" => Some(TokenCategory::Method),
            "ATAN" => Some(TokenCategory::Method),
            "ATAN2" => Some(TokenCategory::Method),
            "AVG" => Some(TokenCategory::Method),
            "BIN" => Some(TokenCategory::Method),
            "CALL" => Some(TokenCategory::Method),
            "CAST" => Some(TokenCategory::Method),
            "CEIL" => Some(TokenCategory::Method),
            "CEILING" => Some(TokenCategory::Method),
            "CHARACTER_LENGTH" => Some(TokenCategory::Method),
            "CHAR_LENGTH" => Some(TokenCategory::Method),
            "COALESCE" => Some(TokenCategory::Method),
            "CONCAT" => Some(TokenCategory::Method),
            "CONCAT_WS" => Some(TokenCategory::Method),
            "CONNECTION_ID" => Some(TokenCategory::Method),
            "CONV" => Some(TokenCategory::Method),
            "CONVERT" => Some(TokenCategory::Method),
            "COS" => Some(TokenCategory::Method),
            "COT" => Some(TokenCategory::Method),
            "COUNT" => Some(TokenCategory::Method),
            "CURDATE" => Some(TokenCategory::Method),
            "CURRENT_DATE" => Some(TokenCategory::Method),
            "CURRENT_TIME" => Some(TokenCategory::Method),
            "CURRENT_TIMESTAMP" => Some(TokenCategory::Method),
            "CURRENT_USER" => Some(TokenCategory::Method),
            "CURTIME" => Some(TokenCategory::Method),
            "DATEDIFF" => Some(TokenCategory::Method),
            "DATE_ADD" => Some(TokenCategory::Method),
            "DATE_FORMAT" => Some(TokenCategory::Method),
            "DATE_SUB" => Some(TokenCategory::Method),
            "DAY" => Some(TokenCategory::Method),
            "DAYNAME" => Some(TokenCategory::Method),
            "DAYOFMONTH" => Some(TokenCategory::Method),
            "DAYOFWEEK" => Some(TokenCategory::Method),
            "DAYOFYEAR" => Some(TokenCategory::Method),
            "DEGREES" => Some(TokenCategory::Method),
            "DIV" => Some(TokenCategory::Method),
            "EXEC" => Some(TokenCategory::Method),
            "EXECUTE" => Some(TokenCategory::Method),
            "EXP" => Some(TokenCategory::Method),
            "EXTRACT" => Some(TokenCategory::Method),
            "FIND_IN_SET" => Some(TokenCategory::Method),
            "FIRST" => Some(TokenCategory::Method),
            "FIRST_VALUE" => Some(TokenCategory::Method),
            "FLOOR" => Some(TokenCategory::Method),
            "FROM_DAYS" => Some(TokenCategory::Method),
            "GETDATE" => Some(TokenCategory::Method),
            "GREATEST" => Some(TokenCategory::Method),
            "HOUR" => Some(TokenCategory::Method),
            "IF" => Some(TokenCategory::Method),
            "IFNULL" => Some(TokenCategory::Method),
            "IIF" => Some(TokenCategory::Method),
            "INSTR" => Some(TokenCategory::Method),
            "ISNULL" => Some(TokenCategory::Method),
            "LAG" => Some(TokenCategory::Method),
            "LAST" => Some(TokenCategory::Method),
            "LAST_DAY" => Some(TokenCategory::Method),
            "LAST_INSERT_ID" => Some(TokenCategory::Method),
            "LAST_VALUE" => Some(TokenCategory::Method),
            "LCASE" => Some(TokenCategory::Method),
            "LEAD" => Some(TokenCategory::Method),
            "LEAST" => Some(TokenCategory::Method),
            "LEFT" => Some(TokenCategory::Method),
            "LENGTH" => Some(TokenCategory::Method),
            "LN" => Some(TokenCategory::Method),
            "LOCALTIME" => Some(TokenCategory::Method),
            "LOCALTIMESTAMP" => Some(TokenCategory::Method),
            "LOCATE" => Some(TokenCategory::Method),
            "LOG" => Some(TokenCategory::Method),
            "LOG10" => Some(TokenCategory::Method),
            "LOG2" => Some(TokenCategory::Method),
            "LOWER" => Some(TokenCategory::Method),
            "LPAD" => Some(TokenCategory::Method),
            "LTRIM" => Some(TokenCategory::Method),
            "MAKEDATE" => Some(TokenCategory::Method),
            "MAKETIME" => Some(TokenCategory::Method),
            "MAX" => Some(TokenCategory::Method),
            "MICROSECOND" => Some(TokenCategory::Method),
            "MID" => Some(TokenCategory::Method),
            "MIN" => Some(TokenCategory::Method),
            "MINUTE" => Some(TokenCategory::Method),
            "MOD" => Some(TokenCategory::Method),
            "MONTH" => Some(TokenCategory::Method),
            "MONTHNAME" => Some(TokenCategory::Method),
            "NEWID" => Some(TokenCategory::Method),
            "NOW" => Some(TokenCategory::Method),
            "NULLIF" => Some(TokenCategory::Method),
            "PERIOD_ADD" => Some(TokenCategory::Method),
            "PERIOD_DIFF" => Some(TokenCategory::Method),
            "PI" => Some(TokenCategory::Method),
            "POSITION" => Some(TokenCategory::Method),
            "POW" => Some(TokenCategory::Method),
            "POWER" => Some(TokenCategory::Method),
            "QUARTER" => Some(TokenCategory::Method),
            "RADIANS" => Some(TokenCategory::Method),
            "RAND" => Some(TokenCategory::Method),
            "REPEAT" => Some(TokenCategory::Method),
            "REPLACE" => Some(TokenCategory::Method),
            "REVERSE" => Some(TokenCategory::Method),
            "RIGHT" => Some(TokenCategory::Method),
            "ROUND" => Some(TokenCategory::Method),
            "ROW_NUMBER" => Some(TokenCategory::Method),
            "RPAD" => Some(TokenCategory::Method),
            "RTRIM" => Some(TokenCategory::Method),
            "SECOND" => Some(TokenCategory::Method),
            "SEC_TO_TIME" => Some(TokenCategory::Method),
            "SESSION_USER" => Some(TokenCategory::Method),
            "SIGN" => Some(TokenCategory::Method),
            "SIN" => Some(TokenCategory::Method),
            "SPACE" => Some(TokenCategory::Method),
            "SQRT" => Some(TokenCategory::Method),
            "STRCMP" => Some(TokenCategory::Method),
            "STR_TO_DATE" => Some(TokenCategory::Method),
            "SUBDATE" => Some(TokenCategory::Method),
            "SUBSTR" => Some(TokenCategory::Method),
            "SUBSTRING" => Some(TokenCategory::Method),
            "SUBSTRING_INDEX" => Some(TokenCategory::Method),
            "SUBTIME" => Some(TokenCategory::Method),
            "SUM" => Some(TokenCategory::Method),
            "SYSDATE" => Some(TokenCategory::Method),
            "SYSTEM_USER" => Some(TokenCategory::Method),
            "TAN" => Some(TokenCategory::Method),
            "TIMEDIFF" => Some(TokenCategory::Method),
            "TIME_FORMAT" => Some(TokenCategory::Method),
            "TIME_TO_SEC" => Some(TokenCategory::Method),
            "TO_DAYS" => Some(TokenCategory::Method),
            "TRIM" => Some(TokenCategory::Method),
            "TRUNCATE" => Some(TokenCategory::Method),
            "UCASE" => Some(TokenCategory::Method),
            "UPPER" => Some(TokenCategory::Method),
            "USER" => Some(TokenCategory::Method),
            "VERSION" => Some(TokenCategory::Method),
            "WEEK" => Some(TokenCategory::Method),
            "WEEKDAY" => Some(TokenCategory::Method),
            "WEEKOFYEAR" => Some(TokenCategory::Method),
            "YEARWEEK" => Some(TokenCategory::Method),
            _ => None,
        };
    }

    fn set_behavior(&mut self) {
        let mut behavior: Vec<TokenBehavior> = vec![];

        match self.category {
            Some(TokenCategory::ParenOpen) => {
                behavior.push(TokenBehavior::NoSpaceAfter);
                behavior.push(TokenBehavior::IncreaseIndent);
            }
            Some(TokenCategory::ParenClose) => behavior.push(TokenBehavior::NoSpaceBefore),
            Some(TokenCategory::Comma) => behavior.push(TokenBehavior::NoSpaceBefore),
            Some(TokenCategory::Comment) => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NewLineAfter);
            }
            _ => (),
        }

        match self.value.to_uppercase().as_str() {
            "AFTER" => behavior.push(TokenBehavior::NewLineBefore),
            "AND" => behavior.push(TokenBehavior::NewLineBefore),
            "BEFORE" => behavior.push(TokenBehavior::NewLineBefore),
            "BEGIN" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
            }
            "CALL" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndentOnSingleLine);
            }
            "CASE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NewLineAfter);
                behavior.push(TokenBehavior::IncreaseIndent);
            }
            "CATCH" => behavior.push(TokenBehavior::NewLineAfter),
            "CLOSE" => behavior.push(TokenBehavior::NewLineBefore),
            "CROSS" => behavior.push(TokenBehavior::NewLineBefore),
            "DECLARE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndentOnSingleLine);
            }
            "DELETE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
            }
            "DISTINCT" => behavior.push(TokenBehavior::NewLineAfter),
            "DO" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NewLineAfter);
                behavior.push(TokenBehavior::IncreaseIndent);
            }
            "DROP" => behavior.push(TokenBehavior::NewLineBefore),
            "ELSE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
            }
            "END" => behavior.push(TokenBehavior::NewLineBefore),
            "EXEC" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndentOnSingleLine);
            }
            "EXECUTE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndentOnSingleLine);
            }
            "FETCH" => behavior.push(TokenBehavior::NewLineBefore),
            "FOR" => behavior.push(TokenBehavior::NewLineBefore),
            "FROM" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
            }
            "GROUP" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
            }
            "HAVING" => behavior.push(TokenBehavior::IncreaseIndent),
            "INNER" => behavior.push(TokenBehavior::NewLineBefore),
            "INSERT" => behavior.push(TokenBehavior::IncreaseIndent),
            "INTO" => behavior.push(TokenBehavior::IncreaseIndent),
            "JOIN" => {
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndentIfFound);
            }
            "LEFT" => behavior.push(TokenBehavior::NewLineBefore),
            "LIMIT" => behavior.push(TokenBehavior::NewLineBefore),
            "OPEN" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
            }
            "OR" => behavior.push(TokenBehavior::NewLineBefore),
            "ORDER" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
            }
            "OUTER" => behavior.push(TokenBehavior::NewLineBefore),
            "PRIMARY" => behavior.push(TokenBehavior::NewLineBefore),
            "RETURN" => behavior.push(TokenBehavior::NewLineBefore),
            "RIGHT" => behavior.push(TokenBehavior::NewLineBefore),
            "SELECT" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
            }
            "SET" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndentOnSingleLine);
            }
            "UNION" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NewLineAfter);
            }
            "UPDATE" => behavior.push(TokenBehavior::IncreaseIndent),
            "VALUE" => behavior.push(TokenBehavior::IncreaseIndent),
            "VALUES" => behavior.push(TokenBehavior::IncreaseIndent),
            "WHEN" => behavior.push(TokenBehavior::NewLineBefore),
            "WHERE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
            }
            "WHILE" => behavior.push(TokenBehavior::IncreaseIndent),
            "WITH" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
            }
            _ => (),
        };

        self.behavior = behavior;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenCategory {
    Space,
    Comment,
    Quote,
    NewLine,
    Delimiter,
    Comma,
    ParenOpen,
    ParenClose,
    Operator,
    Bitwise,
    Compare,
    Keyword,
    DataType,
    Method,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenBehavior {
    NewLineBefore,
    NewLineAfter,
    NoSpaceBefore,
    NoSpaceAfter,
    IncreaseIndent,
    DecreaseIndentIfFound,
    DecreaseIndentOnSingleLine,
}

#[derive(Clone)]
enum CommentCategory {
    SingleLine,
    MultiLine,
}

#[derive(Clone)]
enum QuoteCategory {
    Backtick,
    QuoteSingle,
    QuoteDouble,
    Bracket,
    CurlyBracket,
}

pub fn get_sql_tokens(sql: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut curr_token: Token = Token::new();
    let mut in_comment: Option<CommentCategory> = None;
    let mut in_quote: Option<QuoteCategory> = None;

    let sql_bytes: &[u8] = sql.as_bytes();
    for i in 0..sql_bytes.len() {
        let curr_ch: char = sql_bytes[i].into();

        let prev2_ch: Option<char> = if i >= 2 {
            Some(sql_bytes[i - 2].into())
        } else {
            None
        };
        let prev_ch: Option<char> = if i >= 1 {
            Some(sql_bytes[i - 1].into())
        } else {
            None
        };
        let next_ch: Option<char> = if (i + 1) < sql_bytes.len() {
            Some(sql_bytes[i + 1].into())
        } else {
            None
        };

        let was_in_comment: Option<CommentCategory> = in_comment.clone();
        in_comment = get_in_comment(
            &in_comment,
            prev2_ch,
            prev_ch,
            curr_ch,
            next_ch,
            curr_token.len(),
        );
        if in_comment.is_some() {
            if was_in_comment.is_none() {
                // start of new comment, add any current token if any
                if !curr_token.is_empty() {
                    curr_token.setup();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.category = Some(TokenCategory::Comment);
                curr_token.setup();
            }

            curr_token.value.push(curr_ch);
            continue;
        } else if was_in_comment.is_some() {
            // comment just ended, add comment token
            tokens.push(curr_token);
            curr_token = Token::new();
        }

        let was_in_quote: Option<QuoteCategory> = in_quote.clone();
        in_quote = get_in_quote(&in_quote, prev_ch, curr_ch, next_ch, &curr_token);
        if in_quote.is_some() {
            if was_in_quote.is_none() {
                // start of new quote, add any current token if any
                if !curr_token.is_empty() {
                    curr_token.setup();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.category = Some(TokenCategory::Quote);
                curr_token.setup();
            }

            curr_token.value.push(curr_ch);
            continue;
        } else if was_in_quote.is_some() {
            // quote just ended, add quote token
            tokens.push(curr_token);
            curr_token = Token::new();
        }

        match curr_ch {
            DELIMITER | NEW_LINE | COMMA | PAREN_OPEN | PAREN_CLOSE | AMPERSAND | VERTICAL_BAR
            | CIRCUMFLEX => {
                if !curr_token.is_empty() {
                    curr_token.setup();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.value.push(curr_ch);
                curr_token.setup();
                tokens.push(curr_token);
                curr_token = Token::new();
                continue;
            }
            LESS_THAN | ASTERISK | SLASH_FORWARD | PERCENT => {
                if !curr_token.is_empty() {
                    curr_token.setup();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.value.push(curr_ch);
                curr_token.setup();

                if next_ch != Some(EQUAL) && next_ch != Some(GREATER_THAN) {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }

                continue;
            }
            EQUAL | GREATER_THAN => {
                if !curr_token.is_empty()
                    && prev_ch != Some(LESS_THAN)
                    && prev_ch != Some(GREATER_THAN)
                    && prev_ch != Some(PLUS)
                    && prev_ch != Some(HYPHEN)
                    && prev_ch != Some(ASTERISK)
                    && prev_ch != Some(SLASH_FORWARD)
                    && prev_ch != Some(PERCENT)
                {
                    curr_token.setup();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.value.push(curr_ch);
                curr_token.category = match prev_ch {
                    Some(PLUS) => Some(TokenCategory::Operator),
                    Some(HYPHEN) => Some(TokenCategory::Operator),
                    Some(ASTERISK) => Some(TokenCategory::Operator),
                    Some(SLASH_FORWARD) => Some(TokenCategory::Operator),
                    Some(PERCENT) => Some(TokenCategory::Operator),
                    _ => Some(TokenCategory::Compare),
                };
                curr_token.setup();

                if next_ch != Some(EQUAL) {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }

                continue;
            }
            PLUS | HYPHEN => {
                if !curr_token.is_empty() {
                    curr_token.setup();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.value.push(curr_ch);

                if tokens.last().is_some_and(|t| t.category.is_some()) {
                    continue;
                }

                curr_token.setup();

                if next_ch != Some(EQUAL) && next_ch != Some(GREATER_THAN) {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }

                continue;
            }
            _ => (),
        }

        if curr_ch.is_whitespace() {
            if !curr_token.is_empty() {
                curr_token.setup();
                tokens.push(curr_token);
                curr_token = Token::new();
            }
            continue;
        }

        curr_token.value.push(curr_ch);
    }

    if !curr_token.is_empty() {
        curr_token.setup();
        tokens.push(curr_token);
    }

    return tokens;
}

fn get_in_comment(
    in_comment: &Option<CommentCategory>,
    prev2_ch: Option<char>,
    prev_ch: Option<char>,
    curr_ch: char,
    next_ch: Option<char>,
    curr_token_len: usize,
) -> Option<CommentCategory> {
    match in_comment {
        Some(cc) => {
            if curr_token_len <= 1 {
                return in_comment.clone();
            }

            match cc {
                CommentCategory::SingleLine => {
                    if curr_ch == NEW_LINE {
                        return None;
                    }
                    return Some(CommentCategory::SingleLine);
                }
                CommentCategory::MultiLine => {
                    if prev2_ch == Some(ASTERISK) && prev_ch == Some(SLASH_FORWARD) {
                        return None;
                    }
                    return Some(CommentCategory::MultiLine);
                }
            }
        }
        None => {
            if curr_ch == HYPHEN && next_ch == Some(HYPHEN) {
                return Some(CommentCategory::SingleLine);
            }

            if curr_ch == SLASH_FORWARD && next_ch == Some(ASTERISK) {
                return Some(CommentCategory::MultiLine);
            }

            return None;
        }
    }
}

fn get_in_quote(
    in_quote: &Option<QuoteCategory>,
    prev_ch: Option<char>,
    curr_ch: char,
    next_ch: Option<char>,
    curr_token: &Token,
) -> Option<QuoteCategory> {
    match in_quote {
        Some(qc) => {
            if curr_token.len() <= 1 {
                return in_quote.clone();
            }

            match qc {
                QuoteCategory::Backtick => {
                    if prev_ch == Some(BACKTICK) {
                        return None;
                    }
                    return in_quote.clone();
                }
                QuoteCategory::QuoteSingle => {
                    if prev_ch == Some(QUOTE_SINGLE) && curr_ch != QUOTE_SINGLE {
                        if curr_token.count(QUOTE_SINGLE) % 2 == 0 {
                            return None;
                        }
                    }
                    return in_quote.clone();
                }
                QuoteCategory::QuoteDouble => {
                    if prev_ch == Some(QUOTE_DOUBLE) {
                        return None;
                    }
                    return in_quote.clone();
                }
                QuoteCategory::Bracket => {
                    if prev_ch == Some(BRACKET_CLOSE) && curr_ch != FULL_STOP {
                        return None;
                    }
                    return in_quote.clone();
                }
                QuoteCategory::CurlyBracket => {
                    if prev_ch == Some(CURLY_BRACKET_CLOSE) {
                        return None;
                    }
                    return in_quote.clone();
                }
            }
        }
        None => {
            return match curr_ch {
                BACKTICK => Some(QuoteCategory::Backtick),
                QUOTE_SINGLE => Some(QuoteCategory::QuoteSingle),
                QUOTE_DOUBLE => Some(QuoteCategory::QuoteDouble),
                BRACKET_OPEN => Some(QuoteCategory::Bracket),
                CURLY_BRACKET_OPEN => Some(QuoteCategory::CurlyBracket),
                'N' => {
                    if next_ch == Some(QUOTE_SINGLE) {
                        return Some(QuoteCategory::QuoteSingle);
                    }
                    return None;
                }
                _ => None,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sql_tokens_basic() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT * FROM TBL1")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("FROM"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("TBL1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_single_inline() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT 1 --comment inline")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("--comment inline"),
                    category: Some(TokenCategory::Comment),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::NewLineAfter],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_single_newline() {
        assert_eq!(
            get_sql_tokens(String::from(
                r#"SELECT *
                -- comment newline
                FROM TBL1"#
            )),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("\n"),
                    category: Some(TokenCategory::NewLine),
                    behavior: vec![],
                },
                Token {
                    value: String::from("-- comment newline"),
                    category: Some(TokenCategory::Comment),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::NewLineAfter],
                },
                Token {
                    value: String::from("\n"),
                    category: Some(TokenCategory::NewLine),
                    behavior: vec![],
                },
                Token {
                    value: String::from("FROM"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("TBL1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_multi_inline() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT * /*multi inline*/ FROM TBL1")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("/*multi inline*/"),
                    category: Some(TokenCategory::Comment),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::NewLineAfter],
                },
                Token {
                    value: String::from("FROM"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("TBL1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_multi_odd() {
        assert_eq!(
            get_sql_tokens(String::from("*/*multi odd*/*")),
            vec![
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("/*multi odd*/"),
                    category: Some(TokenCategory::Comment),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::NewLineAfter],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_multi_newline() {
        assert_eq!(
            get_sql_tokens(String::from(
                r#"SELECT *
                /*
                    multi line
                    comment
                */
                FROM TBL1"#
            )),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("\n"),
                    category: Some(TokenCategory::NewLine),
                    behavior: vec![],
                },
                Token {
                    value: String::from(
                        r#"/*
                    multi line
                    comment
                */"#
                    ),
                    category: Some(TokenCategory::Comment),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::NewLineAfter],
                },
                Token {
                    value: String::from("\n"),
                    category: Some(TokenCategory::NewLine),
                    behavior: vec![],
                },
                Token {
                    value: String::from("FROM"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("TBL1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_backtick() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT `Column 1`")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("`Column 1`"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_single() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT 'Column 1'")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("'Column 1'"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_double() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT \"Column 1\"")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("\"Column 1\""),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_bracket() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT [Column 1]")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("[Column 1]"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_bracket_schema() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT * FROM [S].[TBL1]")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("FROM"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("[S].[TBL1]"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_curly_bracket() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT C1 FROM {tableNames[i]}")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("FROM"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("{tableNames[i]}"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_empty() {
        assert_eq!(
            get_sql_tokens(String::from("DECLARE V1 = '';")),
            vec![
                Token {
                    value: String::from("DECLARE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![
                        TokenBehavior::NewLineBefore,
                        TokenBehavior::IncreaseIndent,
                        TokenBehavior::DecreaseIndentOnSingleLine
                    ],
                },
                Token {
                    value: String::from("V1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("''"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
                Token {
                    value: String::from(";"),
                    category: Some(TokenCategory::Delimiter),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_of_empty() {
        assert_eq!(
            get_sql_tokens(String::from("DECLARE V1 = '''';")),
            vec![
                Token {
                    value: String::from("DECLARE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![
                        TokenBehavior::NewLineBefore,
                        TokenBehavior::IncreaseIndent,
                        TokenBehavior::DecreaseIndentOnSingleLine
                    ],
                },
                Token {
                    value: String::from("V1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("''''"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
                Token {
                    value: String::from(";"),
                    category: Some(TokenCategory::Delimiter),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_single_n() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT N'Column Name'")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("N'Column Name'"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_single_escape() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT 'Column''s Name'")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("'Column''s Name'"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_single_multiline() {
        assert_eq!(
            get_sql_tokens(String::from(
                r#"SELECT 'Column
Name'"#
            )),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from(
                        r#"'Column
Name'"#
                    ),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_single_abrupt_end() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT 'Column")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("'Column"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_delimiter_basic() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT 1;")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(";"),
                    category: Some(TokenCategory::Delimiter),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_delimiter_two() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT 1; SELECT 1;")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(";"),
                    category: Some(TokenCategory::Delimiter),
                    behavior: vec![],
                },
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(";"),
                    category: Some(TokenCategory::Delimiter),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_comma() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT 1,2, 3")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(","),
                    category: Some(TokenCategory::Comma),
                    behavior: vec![TokenBehavior::NoSpaceBefore],
                },
                Token {
                    value: String::from("2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(","),
                    category: Some(TokenCategory::Comma),
                    behavior: vec![TokenBehavior::NoSpaceBefore],
                },
                Token {
                    value: String::from("3"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_negative_number() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT -1")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("-1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_empty() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT MIN()")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("MIN"),
                    category: Some(TokenCategory::Method),
                    behavior: vec![],
                },
                Token {
                    value: String::from("("),
                    category: Some(TokenCategory::ParenOpen),
                    behavior: vec![TokenBehavior::NoSpaceAfter, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from(")"),
                    category: Some(TokenCategory::ParenClose),
                    behavior: vec![TokenBehavior::NoSpaceBefore],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_content() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT (SELECT 1)")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("("),
                    category: Some(TokenCategory::ParenOpen),
                    behavior: vec![TokenBehavior::NoSpaceAfter, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(")"),
                    category: Some(TokenCategory::ParenClose),
                    behavior: vec![TokenBehavior::NoSpaceBefore],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_add() {
        assert_eq!(
            get_sql_tokens(String::from("1+2 + 3")),
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("+"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("+"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("3"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_subtract() {
        assert_eq!(
            get_sql_tokens(String::from("1-2 - 3")),
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("-"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("-"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("3"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_multiply() {
        assert_eq!(
            get_sql_tokens(String::from("1*2 * 3")),
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("3"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_divide() {
        assert_eq!(
            get_sql_tokens(String::from("1/2 / 3")),
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("/"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("/"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("3"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_modulo() {
        assert_eq!(
            get_sql_tokens(String::from("1%2 % 3")),
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("%"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("%"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("3"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_add_equal() {
        assert_eq!(
            get_sql_tokens(String::from("V+=1")),
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("+="),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_minus_equal() {
        assert_eq!(
            get_sql_tokens(String::from("V-=1")),
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("-="),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_multiply_equal() {
        assert_eq!(
            get_sql_tokens(String::from("V*=1")),
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("*="),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_divide_equal() {
        assert_eq!(
            get_sql_tokens(String::from("V/=1")),
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("/="),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_modulo_equal() {
        assert_eq!(
            get_sql_tokens(String::from("V%=1")),
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("%="),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_bitwise_and() {
        assert_eq!(
            get_sql_tokens(String::from("V1&V2")),
            vec![
                Token {
                    value: String::from("V1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("&"),
                    category: Some(TokenCategory::Bitwise),
                    behavior: vec![],
                },
                Token {
                    value: String::from("V2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_bitwise_or() {
        assert_eq!(
            get_sql_tokens(String::from("V1|V2")),
            vec![
                Token {
                    value: String::from("V1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("|"),
                    category: Some(TokenCategory::Bitwise),
                    behavior: vec![],
                },
                Token {
                    value: String::from("V2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_bitwise_exclusive_or() {
        assert_eq!(
            get_sql_tokens(String::from("V1^V2")),
            vec![
                Token {
                    value: String::from("V1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("^"),
                    category: Some(TokenCategory::Bitwise),
                    behavior: vec![],
                },
                Token {
                    value: String::from("V2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_lt() {
        assert_eq!(
            get_sql_tokens(String::from("WHERE C1<C2 AND C1 < C2")),
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("<"),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("AND"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("<"),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_gt() {
        assert_eq!(
            get_sql_tokens(String::from("WHERE C1>C2 AND C1 > C2")),
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(">"),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("AND"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(">"),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_eq() {
        assert_eq!(
            get_sql_tokens(String::from("WHERE C1=C2 AND C1 = C2")),
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("AND"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_neq() {
        assert_eq!(
            get_sql_tokens(String::from("WHERE C1<>C2 AND C1 <> C2")),
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("<>"),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("AND"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("<>"),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_gteq() {
        assert_eq!(
            get_sql_tokens(String::from("WHERE C1>=C2 AND C1 >= C2")),
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(">="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("AND"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(">="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_lteq() {
        assert_eq!(
            get_sql_tokens(String::from("WHERE C1<=C2 AND C1 <= C2")),
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore, TokenBehavior::IncreaseIndent],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("<="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("AND"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("<="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }
}
