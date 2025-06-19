// src/lib/materials/OutlineSpriteMaterial.ts
import * as THREE from 'three';

// We will rely on the consumer (Svelte component) to pass the correct
// 'spritesheet' object, which has properties like texture, frameInfoTexture, etc.
// No explicit Spritesheet type import from @threlte/extras is needed here.

// This vertex shader is adapted from the InstancedSprite source
// to ensure compatibility with how InstancedSprite manages UVs and instances.
const vertexShader = `
  attribute vec3 instancePosition;
  attribute float instanceAnimationIndex;
  attribute float instanceFrameIndex;

  uniform sampler2D frameInfoTexture;
  uniform float frameInfoTextureWidth;
  uniform float frameInfoTextureHeight;

  uniform mat4 projectionMatrix;
  uniform mat4 viewMatrix;
  uniform mat4 modelMatrix;

  uniform vec2 uvTransform;
  uniform vec2 uvOffset;
  uniform vec2 uvScale;

  uniform bool billboarding;

  varying vec2 vUv;
  varying vec4 vFrame;

  void main() {
    vUv = uv;

    vec2 frameInfoTextureCoord = vec2(
      (instanceAnimationIndex + 0.5) / frameInfoTextureWidth,
      (instanceFrameIndex + 0.5) / frameInfoTextureHeight
    );
    vFrame = texture2D(frameInfoTexture, frameInfoTextureCoord);

    // vFrame contains:
    // vFrame.x = x position of the frame in the atlas
    // vFrame.y = y position of the frame in the atlas
    // vFrame.z = width of the frame
    // vFrame.w = height of the frame

    vec3 transformedPosition = position;
    if (billboarding) {
        transformedPosition = (modelMatrix * vec4(position, 1.0)).xyz;
        vec3 cameraRight = vec3(viewMatrix[0][0], viewMatrix[1][0], viewMatrix[2][0]);
        vec3 cameraUp = vec3(viewMatrix[0][1], viewMatrix[1][1], viewMatrix[2][1]);
        transformedPosition += cameraRight * transformedPosition.x + cameraUp * transformedPosition.y;
    }

    gl_Position = projectionMatrix * viewMatrix * modelMatrix * vec4(transformedPosition + instancePosition, 1.0);
  }
`;

// This fragment shader will calculate the outline
const fragmentShader = `
  precision highp float;

  uniform sampler2D map; // The main spritesheet texture
  uniform vec3 color;    // Base color tint (if any)
  uniform float opacity; // Overall opacity

  uniform float outlineWidth; // In pixels
  uniform vec4 outlineColor; // RGBA color of the outline

  uniform vec4 vFrame; // Frame info from vertex shader
  uniform vec2 uvTransform; // Atlas UV transform
  uniform vec2 uvOffset;    // Atlas UV offset
  uniform vec2 uvScale;     // Atlas UV scale

  varying vec2 vUv; // UV from vertex shader

  void main() {
    // Calculate the actual UV within the specific frame in the atlas
    // This is the UV coordinate relative to the *whole* spritesheet texture
    vec2 fullTextureUv = mod(vUv * uvTransform, 1.0) * vFrame.zw + vFrame.xy;

    // Calculate texture pixel size
    vec2 texelSize = 1.0 / vec2(textureSize(map, 0)); // size of one pixel in UV space

    // Sample the center pixel
    vec4 texColor = texture2D(map, fullTextureUv);

    // Check neighbors for outline
    bool isOutline = false;
    for (float x = -outlineWidth; x <= outlineWidth; x += 1.0) {
      for (float y = -outlineWidth; y <= outlineWidth; y += 1.0) {
        // Skip current pixel
        if (x == 0.0 && y == 0.0) continue;

        vec2 neighborFullTextureUv = fullTextureUv + vec2(x, y) * texelSize;

        // Ensure neighborFullTextureUv is within the current sprite frame boundaries
        // vFrame.xy is the bottom-left corner of the frame in atlas UVs
        // vFrame.zw is the width/height of the frame in atlas UVs
        // This check prevents outlining other sprites in the atlas
        if (neighborFullTextureUv.x >= vFrame.x && neighborFullTextureUv.x <= vFrame.x + vFrame.z &&
            neighborFullTextureUv.y >= vFrame.y && neighborFullTextureUv.y <= vFrame.y + vFrame.w) {
          vec4 neighborTexColor = texture2D(map, neighborFullTextureUv);
          if (neighborTexColor.a > 0.0) { // If the neighbor is opaque (or partially)
            // And the current pixel is transparent or very low alpha
            if (texColor.a < 0.01) { // Threshold for transparency, adjust as needed
              isOutline = true;
              break;
            }
          }
        }
      }
      if (isOutline) break;
    }

    if (isOutline) {
      gl_FragColor = outlineColor;
    } else {
      gl_FragColor = texColor * vec4(color, opacity);
    }

    // Apply alpha test to discard fully transparent pixels (if needed)
    // if (gl_FragColor.a < 0.01) discard;
  }
`;

// Define a type for the spritesheet object structure that we expect
// This is a minimal type based on the properties accessed by the material.
interface InstancedSpriteSpritesheet {
  texture: THREE.Texture;
  frameInfoTexture: THREE.DataTexture;
  uvTransform: THREE.Vector2;
  uvOffset: THREE.Vector2;
  uvScale: THREE.Vector2;
}

export class OutlineSpriteMaterial extends THREE.ShaderMaterial {
  constructor(
    spritesheet: InstancedSpriteSpritesheet, // Use our local interface
    outlineWidth: number,
    outlineColor: THREE.ColorRepresentation,
  ) {
    super({
      uniforms: {
        map: { value: spritesheet.texture },
        color: { value: new THREE.Color(1, 1, 1) }, // Default white tint
        opacity: { value: 1.0 },
        billboarding: { value: true }, // InstancedSprite expects this, so we pass it
        frameInfoTexture: { value: spritesheet.frameInfoTexture },
        frameInfoTextureWidth: {
          value: spritesheet.frameInfoTexture.image.width,
        },
        frameInfoTextureHeight: {
          value: spritesheet.frameInfoTexture.image.height,
        },
        uvTransform: {
          value: new THREE.Vector2(
            spritesheet.uvTransform.x,
            spritesheet.uvTransform.y,
          ),
        },
        uvOffset: {
          value: new THREE.Vector2(
            spritesheet.uvOffset.x,
            spritesheet.uvOffset.y,
          ),
        },
        uvScale: {
          value: new THREE.Vector2(
            spritesheet.uvScale.x,
            spritesheet.uvScale.y,
          ),
        },

        // Custom outline uniforms
        outlineWidth: { value: outlineWidth },
        outlineColor: {
          value: new THREE.Color(outlineColor).toArray().concat(1.0),
        }, // Convert color to RGBA array [R, G, B, A]
      },
      vertexShader: vertexShader,
      fragmentShader: fragmentShader,
      transparent: true, // Crucial for sprites with alpha
      side: THREE.DoubleSide, // Render both sides
      depthTest: true,
      depthWrite: false, // Prevents transparent parts from writing to depth buffer
      blending: THREE.NormalBlending,
    });
  }
}
