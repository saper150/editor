#version 330 core
in vec2 TexCoords;
in vec3 vertexColor;

layout(location = 0, index = 0) out vec4 outColor;
layout(location = 0, index = 1) out vec4 outAlpha;

uniform sampler2D text;

void main()
{
    vec4 tex_col = texture(text, TexCoords);
    outColor = vec4(vertexColor.xyz, 1.0);
    outAlpha = tex_col;

}
