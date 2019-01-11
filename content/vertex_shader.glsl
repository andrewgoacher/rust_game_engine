#version 150
in vec4 position;
in vec3 normal;
in vec3 texture;
out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;
uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;
void main() {
    v_tex_coords = texture.xy;
    mat4 modelview = view * model;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    gl_Position = perspective * modelview * position;
    v_position = gl_Position.xyz / gl_Position.w;
}