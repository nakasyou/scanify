import * as imagescript from 'https://deno.land/x/imagescript@1.2.17/mod.ts'
import { fromImageData } from '../mod.ts'

console.log('Started')

const targetFile = await Deno.readFile(
  './example/real_target.png',
)

const image = (await imagescript.decode(targetFile)) as imagescript.Image

const data: ImageData = {
  colorSpace: 'srgb',
  data: image.bitmap,
  width: image.width,
  height: image.height
}

const coreImage = fromImageData(data)

const scanedImageData = coreImage.scan([
  { x: 197, y: 688 },
  { x: 1727, y: 735 },
  { x: 1865, y: 3031 },
  { x: 11, y: 3031 }
])

const resultImage = new imagescript.Image(scanedImageData.width, scanedImageData.height)
/*
let firstIndex = 0
for (let x = 0; x !== scanedImageData.width; x++) {
  for (let y = 0; y !== scanedImageData.height; y++) {
    const r = scanedImageData.data[firstIndex]
    const g = scanedImageData.data[firstIndex + 1]
    const b = scanedImageData.data[firstIndex + 2]
    const a = scanedImageData.data[firstIndex + 3]

    console.log(x, y, resultImage.bitmap)
    resultImage.setPixelAt(x, y, imagescript.Image.rgbaToColor(r, g, b, a))

    firstIndex += 4
  }
}*/
resultImage.bitmap = scanedImageData.data

await Deno.writeFile('./example/dist.png', await resultImage.encode())