import SwiftUI

struct USAOutline: Shape {
    func path(in rect: CGRect) -> Path {
        var path = Path()

        // A very rough polygon approximation of the US
        path.move(to: CGPoint(x: rect.width * 0.15, y: rect.height * 0.4))
        path.addLine(to: CGPoint(x: rect.width * 0.35, y: rect.height * 0.35))
        path.addLine(to: CGPoint(x: rect.width * 0.45, y: rect.height * 0.45))
        path.addLine(to: CGPoint(x: rect.width * 0.4, y: rect.height * 0.6))
        path.addLine(to: CGPoint(x: rect.width * 0.2, y: rect.height * 0.55))
        path.closeSubpath()

        return path
    }
}

struct MapView: View {
    @State private var selectedCapital: String? = nil

    var body: some View {
        GeometryReader { geo in
            ZStack {
                Image("WorldMap")
                    .resizable()
                    .scaledToFit()

                USAOutline()
                    .fill(Color.blue)
                    .contentShape(USAOutline()) // Makes it tappable
                    .onTapGesture {
                        selectedCapital = "Washington, D.C."
                    }

                if let capital = selectedCapital {
                    VStack {
                        Spacer()
                        Text("Capital: \(capital)")
                            .padding()
                            .background(Color.white.opacity(0.8))
                            .cornerRadius(10)
                            .padding()
                    }
                }
            }
        }
    }
}

