#version 330 core
layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 dim;
layout (location = 2) in vec3 color;

out vec4 vertexColor;

uniform mat4 projection;

void main()
{

    vec2 v = vec2(
        (gl_VertexID == 0 || gl_VertexID == 1) ? 0. : 1.,
        (gl_VertexID == 1 || gl_VertexID == 2) ? 0. : 1.
    );

    vec2 position = pos + dim * v;

    gl_Position = projection * vec4(position.xy, 0.0, 1.0);
    vertexColor = vec4(color.xyz, 1.0);
}