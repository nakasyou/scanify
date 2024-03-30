import { fromImageData } from '../mod.ts'

; (async () => {
    const targetFile = await fetch('/example/real_target.png').then(res => res.blob())
    const targetImage = await new Promise<HTMLImageElement>((resolve) => {
        const image = new Image()
        image.onload = () => {
            resolve(image)
        }
        image.src = URL.createObjectURL(targetFile)
    })

    const canvas = document.createElement('canvas')
    document.body.append(canvas)
    const ctx = canvas.getContext('2d')!
    canvas.width = targetImage.width
    canvas.height = targetImage.height
    ctx.drawImage(targetImage, 0, 0)

    const data = ctx.getImageData(0, 0, canvas.width, canvas.height)
    const coreImage = fromImageData(data)

    const scanedImageData = coreImage.scan([
        { x: 197, y: 688 },
        { x: 1727, y: 735 },
        { x: 1865, y: 3031 },
        { x: 11, y: 3031 }
    ])


    await ctx.putImageData(scanedImageData, 0, 0)

})()
