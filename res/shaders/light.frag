#version 330 core
out vec4 final_color;

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

struct Light {
    vec3 pos;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in vec3 v_pos;
in vec3 v_normal;

uniform vec3 view_pos;
uniform Material material;
uniform Light light;

void main() {
    vec3 ambient = light.ambient * material.ambient;

    vec3 norm = normalize(v_normal);
    vec3 light_dir = normalize(light.pos - v_pos);
    float diff = max(dot(norm, light_dir), 0.0);

    vec3 diffuse = light.diffuse * (diff * material.diffuse);

    vec3 view_dir = normalize(view_pos - v_pos);
    vec3 halfway_dir = normalize(light_dir + view_dir);
    float spec = pow(max(dot(norm, halfway_dir), 0.0), material.shininess);

    vec3 specular = light.specular * (spec * material.specular);

    final_color = vec4(ambient + diffuse + specular, 1.0);
}
