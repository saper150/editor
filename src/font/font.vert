// #version 330 core
// layout (location = 0) in vec4 vertex; // <vec2 pos, vec2 tex>
// out vec2 TexCoords;

// uniform mat4 projection;

// void main()
// {
//     gl_Position = projection * vec4(vertex.xy, 0.0, 1.0);
//     TexCoords = vertex.zw;
// }

#version 330 core
layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 dim;
layout (location = 2) in vec2 uv_pos;
layout (location = 3) in vec2 uv_dim;
layout (location = 4) in vec3 color;

out vec3 vertexColor;
out vec2 TexCoords;

uniform mat4 projection;

void main()
{

    vec2 v = vec2(
        (gl_VertexID == 0 || gl_VertexID == 1) ? 0. : 1.,
        (gl_VertexID == 1 || gl_VertexID == 2) ? 0. : 1.
    );

    vec2 position = pos + dim * v;
    TexCoords = uv_pos + uv_dim * v;

    vertexColor = color;

    gl_Position = projection * vec4(position.xy, 0.0, 1.0);
}

