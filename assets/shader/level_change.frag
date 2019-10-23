#version 130

in vec2 v_uv;
in vec4 v_color;

out vec4 o_color;

uniform sampler2D u_texture;

uniform float u_progress;
uniform float u_size;

void main() {
    float xFraction = fract(gl_FragCoord.x / u_size);
    float yFraction = fract(gl_FragCoord.y / u_size);

    float xDistance = abs(xFraction - 0.5);
    float yDistance = abs(yFraction - 0.5);

    if (xDistance + yDistance + v_uv.x + v_uv.y < u_progress * 4f) {
        discard;
    }
    o_color = v_color * texture(u_texture, v_uv);

}