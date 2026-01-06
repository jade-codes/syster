#![allow(clippy::unwrap_used)]

use std::collections::HashSet;
use std::path::Path;
use syster::keywords::{
    KERML_KEYWORDS, RELATIONSHIP_OPERATORS, SYSML_KEYWORDS, get_keywords_for_file,
};

/// Test that KERML_KEYWORDS is not empty
#[test]
fn test_kerml_keywords_not_empty() {
    assert!(
        !KERML_KEYWORDS.is_empty(),
        "KERML_KEYWORDS should contain at least one keyword"
    );
}

/// Test that SYSML_KEYWORDS is not empty
#[test]
fn test_sysml_keywords_not_empty() {
    assert!(
        !SYSML_KEYWORDS.is_empty(),
        "SYSML_KEYWORDS should contain at least one keyword"
    );
}

/// Test that RELATIONSHIP_OPERATORS is not empty
#[test]
fn test_relationship_operators_not_empty() {
    assert!(
        !RELATIONSHIP_OPERATORS.is_empty(),
        "RELATIONSHIP_OPERATORS should contain at least one operator"
    );
}

/// Test that KERML_KEYWORDS contains no duplicates
#[test]
fn test_kerml_keywords_no_duplicates() {
    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();

    for keyword in KERML_KEYWORDS {
        if !seen.insert(keyword) {
            duplicates.push(keyword);
        }
    }

    assert!(
        duplicates.is_empty(),
        "Found duplicate keywords in KERML_KEYWORDS: {:?}",
        duplicates
    );
}

/// Test that SYSML_KEYWORDS contains no duplicates
#[test]
fn test_sysml_keywords_no_duplicates() {
    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();

    for keyword in SYSML_KEYWORDS {
        if !seen.insert(keyword) {
            duplicates.push(keyword);
        }
    }

    assert!(
        duplicates.is_empty(),
        "Found duplicate keywords in SYSML_KEYWORDS: {:?}",
        duplicates
    );
}

/// Test that RELATIONSHIP_OPERATORS contains no duplicates
#[test]
fn test_relationship_operators_no_duplicates() {
    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();

    for operator in RELATIONSHIP_OPERATORS {
        if !seen.insert(operator) {
            duplicates.push(operator);
        }
    }

    assert!(
        duplicates.is_empty(),
        "Found duplicate operators in RELATIONSHIP_OPERATORS: {:?}",
        duplicates
    );
}

/// Test that KERML_KEYWORDS contains core KerML keywords
#[test]
fn test_kerml_keywords_contains_core_types() {
    let keywords_set: HashSet<&str> = KERML_KEYWORDS.iter().copied().collect();

    // Test for core type keywords
    assert!(
        keywords_set.contains("classifier"),
        "KERML_KEYWORDS should contain 'classifier'"
    );
    assert!(
        keywords_set.contains("class"),
        "KERML_KEYWORDS should contain 'class'"
    );
    assert!(
        keywords_set.contains("struct"),
        "KERML_KEYWORDS should contain 'struct'"
    );
    assert!(
        keywords_set.contains("datatype"),
        "KERML_KEYWORDS should contain 'datatype'"
    );
    assert!(
        keywords_set.contains("feature"),
        "KERML_KEYWORDS should contain 'feature'"
    );
    assert!(
        keywords_set.contains("package"),
        "KERML_KEYWORDS should contain 'package'"
    );
}

/// Test that KERML_KEYWORDS contains relationship keywords
#[test]
fn test_kerml_keywords_contains_relationships() {
    let keywords_set: HashSet<&str> = KERML_KEYWORDS.iter().copied().collect();

    assert!(
        keywords_set.contains("specializes"),
        "KERML_KEYWORDS should contain 'specializes'"
    );
    assert!(
        keywords_set.contains("subsets"),
        "KERML_KEYWORDS should contain 'subsets'"
    );
    assert!(
        keywords_set.contains("redefines"),
        "KERML_KEYWORDS should contain 'redefines'"
    );
    assert!(
        keywords_set.contains("conjugates"),
        "KERML_KEYWORDS should contain 'conjugates'"
    );
}

/// Test that KERML_KEYWORDS contains feature direction keywords
#[test]
fn test_kerml_keywords_contains_feature_directions() {
    let keywords_set: HashSet<&str> = KERML_KEYWORDS.iter().copied().collect();

    assert!(
        keywords_set.contains("in"),
        "KERML_KEYWORDS should contain 'in'"
    );
    assert!(
        keywords_set.contains("out"),
        "KERML_KEYWORDS should contain 'out'"
    );
    assert!(
        keywords_set.contains("inout"),
        "KERML_KEYWORDS should contain 'inout'"
    );
}

/// Test that KERML_KEYWORDS contains visibility keywords
#[test]
fn test_kerml_keywords_contains_visibility() {
    let keywords_set: HashSet<&str> = KERML_KEYWORDS.iter().copied().collect();

    assert!(
        keywords_set.contains("public"),
        "KERML_KEYWORDS should contain 'public'"
    );
    assert!(
        keywords_set.contains("private"),
        "KERML_KEYWORDS should contain 'private'"
    );
    assert!(
        keywords_set.contains("protected"),
        "KERML_KEYWORDS should contain 'protected'"
    );
}

/// Test that KERML_KEYWORDS contains control flow keywords
#[test]
fn test_kerml_keywords_contains_control_flow() {
    let keywords_set: HashSet<&str> = KERML_KEYWORDS.iter().copied().collect();

    assert!(
        keywords_set.contains("if"),
        "KERML_KEYWORDS should contain 'if'"
    );
    assert!(
        keywords_set.contains("then"),
        "KERML_KEYWORDS should contain 'then'"
    );
    assert!(
        keywords_set.contains("else"),
        "KERML_KEYWORDS should contain 'else'"
    );
    assert!(
        keywords_set.contains("for"),
        "KERML_KEYWORDS should contain 'for'"
    );
}

/// Test that KERML_KEYWORDS contains boolean keywords
#[test]
fn test_kerml_keywords_contains_boolean_keywords() {
    let keywords_set: HashSet<&str> = KERML_KEYWORDS.iter().copied().collect();

    assert!(
        keywords_set.contains("true"),
        "KERML_KEYWORDS should contain 'true'"
    );
    assert!(
        keywords_set.contains("false"),
        "KERML_KEYWORDS should contain 'false'"
    );
    assert!(
        keywords_set.contains("null"),
        "KERML_KEYWORDS should contain 'null'"
    );
}

/// Test that SYSML_KEYWORDS contains definition keywords (multi-word)
#[test]
fn test_sysml_keywords_contains_definition_keywords() {
    let keywords_set: HashSet<&str> = SYSML_KEYWORDS.iter().copied().collect();

    assert!(
        keywords_set.contains("part def"),
        "SYSML_KEYWORDS should contain 'part def'"
    );
    assert!(
        keywords_set.contains("port def"),
        "SYSML_KEYWORDS should contain 'port def'"
    );
    assert!(
        keywords_set.contains("action def"),
        "SYSML_KEYWORDS should contain 'action def'"
    );
    assert!(
        keywords_set.contains("state def"),
        "SYSML_KEYWORDS should contain 'state def'"
    );
    assert!(
        keywords_set.contains("constraint def"),
        "SYSML_KEYWORDS should contain 'constraint def'"
    );
    assert!(
        keywords_set.contains("requirement def"),
        "SYSML_KEYWORDS should contain 'requirement def'"
    );
}

/// Test that SYSML_KEYWORDS contains usage keywords
#[test]
fn test_sysml_keywords_contains_usage_keywords() {
    let keywords_set: HashSet<&str> = SYSML_KEYWORDS.iter().copied().collect();

    assert!(
        keywords_set.contains("part"),
        "SYSML_KEYWORDS should contain 'part'"
    );
    assert!(
        keywords_set.contains("port"),
        "SYSML_KEYWORDS should contain 'port'"
    );
    assert!(
        keywords_set.contains("action"),
        "SYSML_KEYWORDS should contain 'action'"
    );
    assert!(
        keywords_set.contains("state"),
        "SYSML_KEYWORDS should contain 'state'"
    );
    assert!(
        keywords_set.contains("constraint"),
        "SYSML_KEYWORDS should contain 'constraint'"
    );
}

/// Test that SYSML_KEYWORDS contains requirement-related keywords
#[test]
fn test_sysml_keywords_contains_requirement_keywords() {
    let keywords_set: HashSet<&str> = SYSML_KEYWORDS.iter().copied().collect();

    assert!(
        keywords_set.contains("require"),
        "SYSML_KEYWORDS should contain 'require'"
    );
    assert!(
        keywords_set.contains("assume"),
        "SYSML_KEYWORDS should contain 'assume'"
    );
    assert!(
        keywords_set.contains("satisfy"),
        "SYSML_KEYWORDS should contain 'satisfy'"
    );
    assert!(
        keywords_set.contains("verify"),
        "SYSML_KEYWORDS should contain 'verify'"
    );
}

/// Test that SYSML_KEYWORDS contains control node keywords
#[test]
fn test_sysml_keywords_contains_control_nodes() {
    let keywords_set: HashSet<&str> = SYSML_KEYWORDS.iter().copied().collect();

    assert!(
        keywords_set.contains("decide"),
        "SYSML_KEYWORDS should contain 'decide'"
    );
    assert!(
        keywords_set.contains("merge"),
        "SYSML_KEYWORDS should contain 'merge'"
    );
    assert!(
        keywords_set.contains("fork"),
        "SYSML_KEYWORDS should contain 'fork'"
    );
    assert!(
        keywords_set.contains("join"),
        "SYSML_KEYWORDS should contain 'join'"
    );
}

/// Test that SYSML_KEYWORDS contains temporal keywords
#[test]
fn test_sysml_keywords_contains_temporal_keywords() {
    let keywords_set: HashSet<&str> = SYSML_KEYWORDS.iter().copied().collect();

    assert!(
        keywords_set.contains("when"),
        "SYSML_KEYWORDS should contain 'when'"
    );
    assert!(
        keywords_set.contains("at"),
        "SYSML_KEYWORDS should contain 'at'"
    );
    assert!(
        keywords_set.contains("after"),
        "SYSML_KEYWORDS should contain 'after'"
    );
    assert!(
        keywords_set.contains("until"),
        "SYSML_KEYWORDS should contain 'until'"
    );
}

/// Test that SYSML_KEYWORDS contains structural keywords
#[test]
fn test_sysml_keywords_contains_structural_keywords() {
    let keywords_set: HashSet<&str> = SYSML_KEYWORDS.iter().copied().collect();

    assert!(
        keywords_set.contains("package"),
        "SYSML_KEYWORDS should contain 'package'"
    );
    assert!(
        keywords_set.contains("library"),
        "SYSML_KEYWORDS should contain 'library'"
    );
    assert!(
        keywords_set.contains("import"),
        "SYSML_KEYWORDS should contain 'import'"
    );
}

/// Test that RELATIONSHIP_OPERATORS contains all expected operators
#[test]
fn test_relationship_operators_contains_all() {
    assert_eq!(
        RELATIONSHIP_OPERATORS.len(),
        4,
        "RELATIONSHIP_OPERATORS should contain exactly 4 operators"
    );

    let operators_set: HashSet<&str> = RELATIONSHIP_OPERATORS.iter().copied().collect();

    assert!(
        operators_set.contains(":"),
        "RELATIONSHIP_OPERATORS should contain ':' (typing)"
    );
    assert!(
        operators_set.contains(":>"),
        "RELATIONSHIP_OPERATORS should contain ':>' (specialization)"
    );
    assert!(
        operators_set.contains(":>>"),
        "RELATIONSHIP_OPERATORS should contain ':>>' (redefinition)"
    );
    assert!(
        operators_set.contains("::>"),
        "RELATIONSHIP_OPERATORS should contain '::>' (subsetting)"
    );
}

/// Test get_keywords_for_file with .kerml extension
#[test]
fn test_get_keywords_for_file_kerml() {
    let path = Path::new("test.kerml");
    let keywords = get_keywords_for_file(path);

    assert_eq!(
        keywords.len(),
        KERML_KEYWORDS.len(),
        "Should return KERML_KEYWORDS for .kerml files"
    );
    assert_eq!(
        keywords, KERML_KEYWORDS,
        "Should return KERML_KEYWORDS for .kerml files"
    );
}

/// Test get_keywords_for_file with .sysml extension
#[test]
fn test_get_keywords_for_file_sysml() {
    let path = Path::new("test.sysml");
    let keywords = get_keywords_for_file(path);

    assert_eq!(
        keywords.len(),
        SYSML_KEYWORDS.len(),
        "Should return SYSML_KEYWORDS for .sysml files"
    );
    assert_eq!(
        keywords, SYSML_KEYWORDS,
        "Should return SYSML_KEYWORDS for .sysml files"
    );
}

/// Test get_keywords_for_file with unknown extension defaults to SysML
#[test]
fn test_get_keywords_for_file_unknown_extension() {
    let path = Path::new("test.txt");
    let keywords = get_keywords_for_file(path);

    assert_eq!(
        keywords, SYSML_KEYWORDS,
        "Should default to SYSML_KEYWORDS for unknown extensions"
    );
}

/// Test get_keywords_for_file with no extension defaults to SysML
#[test]
fn test_get_keywords_for_file_no_extension() {
    let path = Path::new("test");
    let keywords = get_keywords_for_file(path);

    assert_eq!(
        keywords, SYSML_KEYWORDS,
        "Should default to SYSML_KEYWORDS when no extension"
    );
}

/// Test get_keywords_for_file with uppercase extension
#[test]
fn test_get_keywords_for_file_uppercase_extension() {
    let path = Path::new("test.KERML");
    let keywords = get_keywords_for_file(path);

    // The function is case-sensitive, so uppercase should default to SysML
    assert_eq!(
        keywords, SYSML_KEYWORDS,
        "Should default to SYSML_KEYWORDS for uppercase extension (case-sensitive)"
    );
}

/// Test get_keywords_for_file with path containing directories
#[test]
fn test_get_keywords_for_file_with_path() {
    let path = Path::new("/some/directory/test.kerml");
    let keywords = get_keywords_for_file(path);

    assert_eq!(
        keywords, KERML_KEYWORDS,
        "Should correctly identify .kerml extension in full path"
    );
}

/// Test get_keywords_for_file with nested path and .sysml extension
#[test]
fn test_get_keywords_for_file_nested_path_sysml() {
    let path = Path::new("./project/models/vehicle.sysml");
    let keywords = get_keywords_for_file(path);

    assert_eq!(
        keywords, SYSML_KEYWORDS,
        "Should correctly identify .sysml extension in nested path"
    );
}

/// Test that all keywords in KERML_KEYWORDS are non-empty strings
#[test]
fn test_kerml_keywords_all_non_empty() {
    for keyword in KERML_KEYWORDS {
        assert!(
            !keyword.is_empty(),
            "All KERML_KEYWORDS should be non-empty strings"
        );
    }
}

/// Test that all keywords in SYSML_KEYWORDS are non-empty strings
#[test]
fn test_sysml_keywords_all_non_empty() {
    for keyword in SYSML_KEYWORDS {
        assert!(
            !keyword.is_empty(),
            "All SYSML_KEYWORDS should be non-empty strings"
        );
    }
}

/// Test that all operators in RELATIONSHIP_OPERATORS are non-empty strings
#[test]
fn test_relationship_operators_all_non_empty() {
    for operator in RELATIONSHIP_OPERATORS {
        assert!(
            !operator.is_empty(),
            "All RELATIONSHIP_OPERATORS should be non-empty strings"
        );
    }
}

/// Test that KERML_KEYWORDS has reasonable size (not too small or too large)
#[test]
fn test_kerml_keywords_reasonable_size() {
    let count = KERML_KEYWORDS.len();
    assert!(
        count >= 50,
        "KERML_KEYWORDS should contain at least 50 keywords, got {}",
        count
    );
    assert!(
        count <= 200,
        "KERML_KEYWORDS should not contain more than 200 keywords, got {}",
        count
    );
}

/// Test that SYSML_KEYWORDS has reasonable size (not too small or too large)
#[test]
fn test_sysml_keywords_reasonable_size() {
    let count = SYSML_KEYWORDS.len();
    assert!(
        count >= 50,
        "SYSML_KEYWORDS should contain at least 50 keywords, got {}",
        count
    );
    assert!(
        count <= 200,
        "SYSML_KEYWORDS should not contain more than 200 keywords, got {}",
        count
    );
}

/// Test that SYSML_KEYWORDS contains common def keywords
#[test]
fn test_sysml_def_keywords_comprehensive() {
    let keywords_set: HashSet<&str> = SYSML_KEYWORDS.iter().copied().collect();

    let expected_defs = vec![
        "part def",
        "port def",
        "action def",
        "state def",
        "constraint def",
        "requirement def",
        "attribute def",
        "connection def",
        "interface def",
        "allocation def",
        "item def",
        "occurrence def",
        "analysis def",
        "case def",
        "verification def",
        "view def",
        "viewpoint def",
        "rendering def",
        "metadata def",
        "enum def",
    ];

    for def_keyword in expected_defs {
        assert!(
            keywords_set.contains(def_keyword),
            "SYSML_KEYWORDS should contain '{}'",
            def_keyword
        );
    }
}

/// Test that KERML and SYSML have some overlap (common keywords)
#[test]
fn test_kerml_sysml_keyword_overlap() {
    let kerml_set: HashSet<&str> = KERML_KEYWORDS.iter().copied().collect();
    let sysml_set: HashSet<&str> = SYSML_KEYWORDS.iter().copied().collect();

    let overlap: Vec<_> = kerml_set.intersection(&sysml_set).collect();

    // There should be some overlap (structural keywords like package, library, etc.)
    assert!(
        !overlap.is_empty(),
        "KERML and SYSML should have some common keywords (e.g., package, library)"
    );

    // Check specific expected overlaps
    assert!(
        kerml_set.contains("package") && sysml_set.contains("package"),
        "Both should contain 'package'"
    );
    assert!(
        kerml_set.contains("import") && sysml_set.contains("import"),
        "Both should contain 'import'"
    );
}

/// Test that SYSML_KEYWORDS has unique keywords not in KERML
#[test]
fn test_sysml_has_unique_keywords() {
    let kerml_set: HashSet<&str> = KERML_KEYWORDS.iter().copied().collect();
    let sysml_set: HashSet<&str> = SYSML_KEYWORDS.iter().copied().collect();

    let sysml_only: Vec<_> = sysml_set.difference(&kerml_set).collect();

    // SysML should have unique keywords like "part def", "action", etc.
    assert!(
        !sysml_only.is_empty(),
        "SYSML should have keywords not in KERML"
    );

    // Check some specific SysML-only keywords
    assert!(
        sysml_set.contains("part") && !kerml_set.contains("part"),
        "SYSML should have 'part' keyword not in KERML"
    );
}

/// Test that keywords don't contain unexpected whitespace
#[test]
fn test_kerml_keywords_whitespace() {
    for keyword in KERML_KEYWORDS {
        assert_eq!(
            keyword.trim(),
            *keyword,
            "KERML keyword '{}' should not have leading/trailing whitespace",
            keyword
        );
    }
}

/// Test that keywords don't contain unexpected whitespace (except multi-word keywords)
#[test]
fn test_sysml_keywords_whitespace() {
    for keyword in SYSML_KEYWORDS {
        assert_eq!(
            keyword.trim(),
            *keyword,
            "SYSML keyword '{}' should not have leading/trailing whitespace",
            keyword
        );

        // Multi-word keywords should have exactly one space between words
        if keyword.contains(' ') {
            assert!(
                !keyword.contains("  "),
                "SYSML keyword '{}' should not contain multiple consecutive spaces",
                keyword
            );
        }
    }
}

/// Test that RELATIONSHIP_OPERATORS doesn't contain whitespace
#[test]
fn test_relationship_operators_no_whitespace() {
    for operator in RELATIONSHIP_OPERATORS {
        assert!(
            !operator.contains(' '),
            "Relationship operator '{}' should not contain whitespace",
            operator
        );
    }
}
