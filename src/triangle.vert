#version 330 core

layout (location = 28) in vec3 Position;

void main()
{
 
    gl_Position = vec4(Position, 1);
}