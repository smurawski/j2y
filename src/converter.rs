use crate::converter::error::ConversionError;

pub fn convert_json_to_yaml(json_str: &str, verbose: bool) -> Result<String, ConversionError> {
    // Parse the string of json data into serde_yaml::Value.
    let v: serde_yaml::Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return Err(ConversionError),
    };
    let yaml_string = serde_yaml::to_string(&v).expect("Failed to convert the YAML to a string.");

    if verbose {
        println!("\nAfter YAML conversion: \n");
        println!("{}", &yaml_string);
    }

    Ok(yaml_string)
}

pub fn convert_yaml_to_json(yaml_str: &str, verbose: bool) -> Result<String, ConversionError> {
    // Parse the string of json data into serde_yaml::Value.
    let v: serde_json::Value = match serde_yaml::from_str(yaml_str) {
        Ok(serde_value) => serde_value,
        Err(_) => return Err(ConversionError),
    };
    let json_string = serde_json::to_string(&v).expect("Failed to convert the JSON to a string.");

    if verbose {
        println!("\nAfter YAML conversion: \n");
        println!("{}", &json_string);
    }

    Ok(json_string)
}

pub mod error {
    use std::fmt;

    pub struct ConversionError;

    impl fmt::Display for ConversionError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Failed to convert the source content to destination content."
            )
        }
    }

    impl fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Failed to convert the source content to destination content."
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_valid_json_to_yaml() {
        let data = r#"{
                    "environment": {},
                    "enabled": true,
                    "continueOnError": false,
                    "alwaysRun": false,
                    "displayName": "PowerShell Script",
                    "timeoutInMinutes": 0,
                    "condition": "succeeded()",
                    "refName": "PowerShell1",
                    "task": {
                        "id": "e213ff0f-5d5c-4791-802d-52ea3e7be1f1",
                        "versionSpec": "1.*",
                        "definitionType": "task"
                    },
                    "inputs": {
                        "scriptType": "filePath",
                        "scriptName": "./build.ps1",
                        "arguments": "",
                        "workingFolder": "",
                        "inlineScript": "",
                        "failOnStandardError": "true"
                    }
                  }"#;
        convert_json_to_yaml(data, false).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_json_to_yaml() {
        let data = r#"{
                    "environment": {},
                    "enabled": true,
                    "continueOnError": false,
                    "alwaysRun": false,
                    "displayName": "PowerShell Script",
                    "timeoutInMinutes": 0,
                    "condition": "succeeded()",
                    "refName": "PowerShell1",
                    "task": {
                        "id": "e213ff0f-5d5c-4791-802d-52ea3e7be1f1",
                        "versionSpec": "1.*",
                        "definitionType": "task"
                    },
                    "inputs": {
                        "scriptType": "filePath",
                        "scriptName": "./build.ps1",
                        "arguments": "",
                        "workingFolder": "",
                        "inlineScript": "",
                        "failOnStandardError": "true"
                    }
                  "#;
        convert_json_to_yaml(data, false).unwrap();
    }

    #[test]
    fn simple_valid_yaml_to_json() {
        let data = r#"---
            environment: {}
            enabled: true
            continueOnError: false
            alwaysRun: false
            displayName: PowerShell Script
            timeoutInMinutes: 0
            condition: succeeded()
            refName: PowerShell1
            task:
                id: "e213ff0f-5d5c-4791-802d-52ea3e7be1f1"
                versionSpec: "1.*"
                definitionType: task
            inputs:
                scriptType: filePath
                scriptName: ./build.ps1
                arguments: ""
                workingFolder: ""
                inlineScript: ""
                failOnStandardError: "true"
        "#;
        convert_yaml_to_json(data, false).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_yaml_to_json() {
        let data = r#"---
            environment: {}
            enabled: true
            continueOnError: false
            alwaysRun: false
            displayName: PowerShell Script
            timeoutInMinutes: 0
            condition: succeeded()
            refName: PowerShell1
            task:
                id: "e213ff0f-5d5c-4791-802d-52ea3e7be1f1"
                versionSpec: "1.*"
                definitionType: task
            inputs:
                scriptType: filePath
                scriptName: ./build.ps1
                arguments: ""
                workingFolder: ""
                inlineScript: ""
                *failOnStandardError: ""
        "#;
        convert_yaml_to_json(data, false).unwrap();
    }
}
