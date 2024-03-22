/**
 * Main module
 * @module
 */

// @deno-types="../pkg/wasm.d.ts"
import * as wasm from '../pkg/wasm.js'

/**
 * Initialize WASM
 * @public
 */
export const initialize = async (): Promise<void> => {
  await wasm.default()
}

interface ImageInit {
  wasmImage: wasm.Image
}

/**
 * Edge positions
 * @example
 * ```
 *   A ___ B
 *   /     \
 *  /       \
 * /_________\
 * D         C
 * ```
 * Then, it be `[A, B, C, D]`.
 * Position is `{ x: number, y: number }`
 * @public
 */
export type Edges = [
  { x: number; y: number },
  { x: number; y: number },
  { x: number; y: number },
  { x: number; y: number },
]

class Image {
  #wasmImage: wasm.Image
  constructor(init: ImageInit) {
    this.#wasmImage = init.wasmImage
  }
  /**
   * Scan via WASM
   * @param edges Edges Data
   * @public
   */
  scan(edges: Edges): ImageData {
    const edgesData = new Uint32Array(edges.flatMap(({ x, y }) => [x, y]))
    const wasmResult = this.#wasmImage.scan(edgesData)
    const scanedImageDataData = new Uint8ClampedArray(wasmResult)
    const imageData = new ImageData(this.#wasmImage.result_width, this.#wasmImage.result_height)
    for (let i = 0; i !== scanedImageDataData.length; i++) {
      imageData.data[i] = scanedImageDataData[i]
    }
    return imageData
  }
}

/**
 * Create Scanify Image from `ImageData`
 * @param imageData ImageData instance
 * @public
 */
export const fromImageData = (imageData: ImageData): Image => {
  const wasmImage = new wasm.Image(
    imageData.width,
    imageData.height,
    imageData.data,
  )

  return new Image({
    wasmImage,
  })
}
