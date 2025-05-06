import SwiftUI

struct CountryShape: Shape {
    let pathData: String

    func path(in rect: CGRect) -> Path {
        var path = Path()

        let tokens = pathData
            .replacingOccurrences(of: "\n", with: " ")
            .split(separator: " ")
            .map(String.init)

        var idx = 0
        var currentPoint = CGPoint.zero
        var startPoint = CGPoint.zero
        var isFirstMove = true

        func nextCoordPair() -> CGPoint? {
            guard idx < tokens.count else { return nil }

            let parts = tokens[idx].split(separator: ",").map(String.init)
            idx += 1

            if parts.count == 2,
               let x = Double(parts[0]),
               let y = Double(parts[1]) {
                return CGPoint(x: x, y: y)
            }
            return nil
        }

        while idx < tokens.count {
            let token = tokens[idx]
            idx += 1

            switch token {
            case "m":
                if let moveTo = nextCoordPair() {
                    if isFirstMove {
                        // First "m": treat as absolute (SVG convention)
                        currentPoint = moveTo
                        isFirstMove = false
                    } else {
                        // Subsequent "m": relative move
                        currentPoint.x += moveTo.x
                        currentPoint.y += moveTo.y
                    }
                    startPoint = currentPoint
                    path.move(to: currentPoint)
                }

            case "z", "Z":
                path.addLine(to: startPoint)
                path.closeSubpath()

            default:
                // Assume it's a coordinate pair (relative line-to)
                idx -= 1 // rewind to handle coordinate
                while let rel = nextCoordPair() {
                    currentPoint.x += rel.x
                    currentPoint.y += rel.y
                    path.addLine(to: currentPoint)
                }
            }
        }

        // Scale to match the SVG viewBox
        let svgWidth: CGFloat = 1009.6727
        let svgHeight: CGFloat = 665.96301

        let scaleX = rect.width / svgWidth
        let scaleY = rect.height / svgHeight
        let transform = CGAffineTransform(scaleX: scaleX, y: scaleY)

        return path.applying(transform)
    }
}
