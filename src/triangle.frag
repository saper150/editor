#version 330 core
in vec2 TexCoords;
out vec4 color;

uniform sampler2D text;

void main()
{
    float sampled = texture(text, TexCoords).r;
    color = vec4(156.0 / 255.0, 220.0 / 255.0, 254.0 / 255.0 , sampled) ;
    // vec4(1.0, 1.0, 1.0, sampled);
}