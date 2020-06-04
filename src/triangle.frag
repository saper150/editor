#version 330 core
in vec2 TexCoords;
// out vec4 color;

layout(location = 0, index = 0) out vec4 outColor;
layout(location = 0, index = 1) out vec4 outAlpha;

uniform sampler2D text;

float gamma = 2.2;

vec4 fromLinear(vec4 color) {
    return pow(color, vec4(gamma));
}

vec3 toLinear(vec3 color) {
    return pow(color, vec3(1.0 / gamma));
}

void main()
{


    vec4 tex_col = texture(text, TexCoords);

    outColor = vec4(213.0 / 255.0, 213.0 / 255.0, 213.0 / 255.0, 1.0);
    outAlpha = tex_col;

    // vec3 v_colour_linear = toLinear(vec3(156.0 / 255.0, 220.0 / 255.0, 254.0 / 255.0));
    //vec3 v_colour_linear = toLinear(vec3(213.0 / 255.0, 213.0 / 255.0, 213.0 / 255.0));
    //vec3 v_background_colour_linear = toLinear(vec3(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0));

    //vec3 v_background_colour_linear = toLinear(vec3(30.0 / 254.0, 30.0 / 254.0, 30.0 / 254.0));

    // vec3 v_background_colour_linear = vec3(0.0, 0.0, 0.0);

	//float r = tex_col.r * v_colour_linear.r + (1.0 - tex_col.r) * v_background_colour_linear.r;
	//float g = tex_col.g * v_colour_linear.g + (1.0 - tex_col.g) * v_background_colour_linear.g;
	//float b = tex_col.b * v_colour_linear.b + (1.0 - tex_col.b) * v_background_colour_linear.b;


    //color = fromLinear(vec4(r, g, b, tex_col.a));


    //color = vec4(156.0 / 255.0, 220.0 / 255.0, 254.0 / 255.0 , sampled);
    // vec4(1.0, 1.0, 1.0, sampled);
}