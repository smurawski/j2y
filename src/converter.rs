extern crate serde_json;
extern crate serde_yaml;

use std::fmt;

pub struct Error {
    pub message: String,
    pub detail: String,
    pub status_code: usize,
    pub source_content: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\nReturn Status Code: {}\nSource Content: \n{}",
            self.message, self.detail, self.status_code, self.source_content
        )
    }
}

pub fn convert_json_to_yaml(json_str: &str, verbose: bool) -> Result<String, Error> {
    // Parse the string of json data into serde_yaml::Value.
    let v: serde_yaml::Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            let wrapped_error = Error {
                message: "Failed to read the content as JSON.".to_string(),
                detail: format!("{:?}", e),
                status_code: 406,
                source_content: json_str.to_string(),
            };
            return Err(wrapped_error);
        }
    };
    let yaml_string = match serde_yaml::to_string(&v) {
        Ok(v) => v,
        Err(e) => {
            let wrapped_error = Error {
                message: "Failed to convert the JSON content into YAML.".to_string(),
                detail: format!("{:?}", e),
                status_code: 500,
                source_content: json_str.to_string(),
            };
            return Err(wrapped_error);
        }
    };

    if verbose {
        println!("\nAfter YAML conversion: \n");
        println!("{}", &yaml_string);
    }

    Ok(yaml_string)
}

pub fn convert_yaml_to_json(yaml_str: &str, verbose: bool) -> Result<String, Error> {
    // Parse the string of json data into serde_yaml::Value.
    let v: serde_json::Value = match serde_yaml::from_str(yaml_str) {
        Ok(v) => v,
        Err(e) => {
            let wrapped_error = Error {
                message: "Failed to read the content as YAML.".to_string(),
                detail: format!("{:?}", e),
                status_code: 406,
                source_content: yaml_str.to_string()
            };
            return Err(wrapped_error);
        }
    };
    let json_string = match serde_json::to_string(&v) {
        Ok(v) => v,
        Err(e) => {
            let wrapped_error = Error {
                message: "Failed to convert the YAML content into JSON.".to_string(),
                detail: format!("{:?}", e),
                status_code: 500,
                source_content: yaml_str.to_string()
            };
            return Err(wrapped_error);
        }
    };

    if verbose {
        println!("\nAfter YAML conversion: \n");
        println!("{}", &json_string);
    }

    Ok(json_string)
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
