import SwiftUI
import AppKit

struct ContentView: View {
    @State private var scale: CGFloat = 1.0
    @State private var offset: CGSize = .zero
    @GestureState private var gestureScale: CGFloat = 1.0
    @GestureState private var gestureDrag: CGSize = .zero
    private let minScale: CGFloat = 1.0
    private let maxScale: CGFloat = 5.0

    @State private var selectedCountry: CountryData? = nil
    @State private var showOnboarding: Bool = true

    var body: some View {
        GeometryReader { geo in
            ZStack {

                Color(red: 226/255, green: 237/255, blue: 1.0)
                    .ignoresSafeArea()

                WorldMapView(selectedCountry: $selectedCountry)
                    .scaleEffect(scale * gestureScale)
                    .offset(
                        x: offset.width + gestureDrag.width,
                        y: offset.height + gestureDrag.height
                    )

                VStack {
                    Spacer()
                    HStack {
                        Spacer()
                        zoomButton(systemImage: "plus.magnifyingglass") {
                            scale = clamp(scale * 1.2, min: minScale, max: maxScale)
                            offset = clampedOffset(afterScale: scale, in: geo.size)
                        }
                        zoomButton(systemImage: "minus.magnifyingglass") {
                            scale = clamp(scale / 1.2, min: minScale, max: maxScale)
                            offset = clampedOffset(afterScale: scale, in: geo.size)
                        }
                        Spacer()
                    }
                    .padding(.horizontal)
                    .padding(.vertical, 4)
                    .background(Color.blue.opacity(0.7))
                }

                // country detail panel
                if let country = selectedCountry {
                    VStack(alignment: .leading, spacing: 12) {
                        HStack {
                            Text(country.name)
                                .font(.title2)
                                .bold()
                                .foregroundColor(.black)
                            Spacer()
                            Button(action: { selectedCountry = nil }) {
                                Image(systemName: "xmark")
                                    .foregroundColor(.gray)
                            }
                            .buttonStyle(.plain)
                        }
                        let numericLevel = country.advisory_level.replacingOccurrences(of: "Level ", with: "")
                        Text("Advisory Level: \(numericLevel)")
                            .font(.headline)
                            .foregroundColor(.black)
                        Divider()
                        ScrollView {
                            VStack(alignment: .leading, spacing: 5) {
                                Text("Traveler’s Checklist (U.S. Department of State)")
                                    .font(.headline)
                                Text("Get Informed – Country info & crisis guidance")
                                Text(" • travel.state.gov/destination")
                                    .foregroundColor(/*@START_MENU_TOKEN@*/.blue/*@END_MENU_TOKEN@*/)
                                Text(" • travel.state.gov/crisis")
                                    .foregroundColor(/*@START_MENU_TOKEN@*/.blue/*@END_MENU_TOKEN@*/)
                                Text("Get Documents – Passport & visas")
                                Text("Get Enrolled – STEP security alerts")
                                Text(" • step.state.gov")
                                    .foregroundColor(/*@START_MENU_TOKEN@*/.blue/*@END_MENU_TOKEN@*/)
                                Text("Get Insured – Medical & evacuation coverage")
                                Text("Key Contacts")
                                    .font(.headline)
                                Text("Overseas Citizens Services: 888-407-4747 (U.S./Canada) | +1 202-501-4444 (overseas)")
                                Text("Passport Info: 877-487-2778 | TTY 888-874-7793")
                                Text("More resources: travel.state.gov/travelerschecklist  |  @TravelGov / travelgov")
                                Text("Special thank you to MapSVG for providing the free SVG world map")
                                Text("• License link: https://creativecommons.org/licenses/by/4.0/")
                            }
                            .foregroundColor(.black)
                            .font(.body)
                        }
                        Spacer()
                    }
                    .padding()
                    .frame(
                        width: geo.size.width * 0.35,
                        height: geo.size.height * 0.7
                    )
                    .background(Color.white)
                    .cornerRadius(10)
                    .shadow(radius: 8)
                    .transition(.move(edge: .trailing))
                    .animation(.easeInOut, value: selectedCountry != nil)
                    .position(
                        x: geo.size.width - (geo.size.width * 0.35 / 2) - 16,
                        y: geo.size.height / 2
                    )
                }

                // onboarding popup
                if showOnboarding {
                    VStack(spacing: 16) {
                        HStack {
                            Spacer()
                            Button(action: { showOnboarding = false }) {
                                Image(systemName: "xmark")
                                    .foregroundColor(.gray)
                            }
                            .buttonStyle(.plain)
                        }
                        Text("Travel Advisory Levels — U.S. Department of State (Bureau of Consular Affairs)")
                            .font(.title3)
                            .bold()
                            .multilineTextAlignment(.center)
                            .foregroundColor(.black)
                        VStack(spacing: 8) {
                            Text("The State Department assigns every country one of four travel-advisory levels:")
                                .multilineTextAlignment(.center)
                                .foregroundColor(.black)
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Level 1: Exercise Normal Precautions")
                                Text("Level 2: Exercise Increased Caution")
                                Text("Level 3: Reconsider Travel")
                                Text("Level 4: Do Not Travel")
                            }
                            .foregroundColor(.black)
                            Text("Click any country on the map to see its current advisory level.")
                                .multilineTextAlignment(.center)
                                .foregroundColor(.black)
                        }
                        Spacer(minLength: 0)
                    }
                    .padding()
                    .frame(
                        width: geo.size.width * 0.6,
                        height: geo.size.height * 0.4
                    )
                    .background(Color.white)
                    .cornerRadius(12)
                    .shadow(radius: 10)
                    .overlay(
                        RoundedRectangle(cornerRadius: 12)
                            .stroke(Color.gray.opacity(0.2), lineWidth: 1)
                    )
                    .position(x: geo.size.width / 2, y: geo.size.height / 2)
                }
            }
            // Attach pinch & pan gestures to entire ZStack
            .gesture(
                SimultaneousGesture(
                    MagnificationGesture()
                        .updating($gestureScale) { val, state, _ in state = val }
                        .onEnded { final in
                            scale = clamp(scale * final, min: minScale, max: maxScale)
                            offset = clampedOffset(afterScale: scale, in: geo.size)
                        },
                    DragGesture()
                        .updating($gestureDrag) { val, state, _ in state = val.translation }
                        .onEnded { val in
                            let t = CGSize(
                                width: offset.width + val.translation.width,
                                height: offset.height + val.translation.height
                            )
                            offset.width = clamp(t.width, forScale: scale, dimension: geo.size.width)
                            offset.height = clamp(t.height, forScale: scale, dimension: geo.size.height)
                        }
                )
            )
            .background(
                ScrollWheelHandler { event in
                    let dx = -event.scrollingDeltaX
                    let dy = event.scrollingDeltaY
                    let t = CGSize(
                        width: offset.width + dx,
                        height: offset.height + dy
                    )
                    offset.width = clamp(t.width, forScale: scale, dimension: geo.size.width)
                    offset.height = clamp(t.height, forScale: scale, dimension: geo.size.height)
                }
            )
        }
    }

    // helper functions
    private func clamp(_ value: CGFloat, min: CGFloat, max: CGFloat) -> CGFloat {
        Swift.max(min, Swift.min(value, max))
    }
    private func clamp(_ value: CGFloat, forScale scale: CGFloat, dimension: CGFloat) -> CGFloat {
        let maxOffset = (scale - 1) * dimension / 2
        return Swift.max(-maxOffset, Swift.min(value, maxOffset))
    }
    private func clampedOffset(afterScale scale: CGFloat, in containerSize: CGSize) -> CGSize {
        CGSize(
            width: clamp(offset.width, forScale: scale, dimension: containerSize.width),
            height: clamp(offset.height, forScale: scale, dimension: containerSize.height)
        )
    }
    @ViewBuilder
    private func zoomButton(systemImage: String, action: @escaping () -> Void) -> some View {
        Button(action: action) {
            Image(systemName: systemImage)
                .imageScale(.large)
                .foregroundColor(.white)
        }
        .buttonStyle(.borderedProminent)
        .fixedSize()
    }
}

// ScrollWheelHandler (for capturing swipe gestures)
struct ScrollWheelHandler: NSViewRepresentable {
    var onScroll: (NSEvent) -> Void
    func makeNSView(context: Context) -> NSView {
        let view = ScrollView()
        view.onScroll = onScroll
        return view
    }
    func updateNSView(_ nsView: NSView, context: Context) {}
    class ScrollView: NSView {
        var onScroll: (NSEvent) -> Void = { _ in }
        override func scrollWheel(with event: NSEvent) {
            super.scrollWheel(with: event)
            onScroll(event)
        }
    }
}

// #Preview {
    // ContentView()
// }

