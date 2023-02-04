#version 330 core

out vec4 outputColor;

void main()
{
    float lerpValue = gl_FragCoord.y / 700.0f;
    
    outputColor = mix(vec4(0.3f, 0.0f, 1.0f, 1.0f),
        vec4(0.2f, 0.2f, 0.2f, 1.0f), lerpValue);
}