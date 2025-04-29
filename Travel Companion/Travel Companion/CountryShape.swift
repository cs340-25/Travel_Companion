import SwiftUI

struct CountryShape: Shape {
    let pathData: String

    func path(in rect: CGRect) -> Path {
        var path = Path()

        let tokens = pathData
            .replacingOccurrences(of: "\n", with: " ")
            .split(separator: " ")
            .map { String($0) }

        var idx = 0
        var currentPoint = CGPoint.zero

        func nextValue() -> CGFloat? {
            guard idx < tokens.count else { return nil }
            let token = tokens[idx]
            idx += 1
            return CGFloat(Double(token) ?? 0)
        }

        while idx < tokens.count {
            let token = tokens[idx]
            idx += 1

            switch token {
            case "M":
                if let x = nextValue(), let y = nextValue() {
                    currentPoint = CGPoint(x: x, y: y)
                    path.move(to: currentPoint)
                }
            case "L":
                if let x = nextValue(), let y = nextValue() {
                    currentPoint = CGPoint(x: x, y: y)
                    path.addLine(to: currentPoint)
                }
            case "l":
                if let dx = nextValue(), let dy = nextValue() {
                    let newPoint = CGPoint(x: currentPoint.x + dx, y: currentPoint.y + dy)
                    path.addLine(to: newPoint)
                    currentPoint = newPoint
                }
            case "Z", "z":
                path.closeSubpath()
            default:
                // Numbers without a command (fallback)
                if let x = Double(token),
                   let y = nextValue().map({ Double($0) }) {
                    let point = CGPoint(x: x, y: y)
                    path.addLine(to: point)
                    currentPoint = point
                }
            }
        }

        // Correct scaling inside a fitted rectangle
        let svgWidth: CGFloat = 2000
        let svgHeight: CGFloat = 857

        let svgAspect = svgWidth / svgHeight
        let rectAspect = rect.width / rect.height

        var drawRect = rect

        if svgAspect > rectAspect {
            let scaledHeight = rect.width / svgAspect
            drawRect = CGRect(
                x: rect.minX,
                y: rect.minY + (rect.height - scaledHeight) / 2,
                width: rect.width,
                height: scaledHeight
            )
        } else {
            let scaledWidth = rect.height * svgAspect
            drawRect = CGRect(
                x: rect.minX + (rect.width - scaledWidth) / 2,
                y: rect.minY,
                width: scaledWidth,
                height: rect.height
            )
        }

        let scaleX = drawRect.width / svgWidth
        let scaleY = drawRect.height / svgHeight
        let transform = CGAffineTransform(scaleX: scaleX, y: scaleY)
            .concatenating(.init(translationX: drawRect.minX, y: drawRect.minY))

        return path.applying(transform)
    }
}
