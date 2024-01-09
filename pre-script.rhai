let project_name = variable::get("project-name");
let class_name = to_upper_camel_case(project_name);
debug(`project_name: ${project_name}`);
debug(`class_name: ${class_name}`);
variable::set("class_name", class_name);

file::rename("src/lib.rs", "src/"+project_name+".rs");
file::rename("src/lib_bench.rs", "src/"+project_name+"_bench.rs");