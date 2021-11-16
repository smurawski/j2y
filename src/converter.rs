extern crate serde_json;
extern crate serde_yaml;

pub fn convert_json_to_yaml(json_str: &str, verbose: bool) -> Result<String, serde_json::Error> {
    // Parse the string of json data into serde_yaml::Value.
    let v: serde_yaml::Value = serde_json::from_str(json_str)?;
    let yaml_string = serde_yaml::to_string(&v).expect("Failed to convert the YAML to a string.");

    if verbose {
        println!("\nAfter YAML conversion: \n");
        println!("{}", &yaml_string);
    }

    Ok(yaml_string)
}

pub fn convert_yaml_to_json(yaml_str: &str, verbose: bool) -> Result<String, serde_yaml::Error> {
    // Parse the string of json data into serde_yaml::Value.
    let v: serde_json::Value = serde_yaml::from_str(yaml_str)?;
    let json_string = serde_json::to_string(&v).expect("Failed to convert the JSON to a string.");

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
