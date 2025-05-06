import SwiftUI

struct USADebugView: View {
    let usa: CountryData?

    init() {
        let allCountries = loadCountries()
        self.usa = allCountries.first(where: { $0.name == "United States" })
    }

    var body: some View {
        GeometryReader { geo in
            ZStack {
                if let usa = usa {
                    CountryShape(pathData: usa.path)
                        .fill(Color.red)
                        .overlay(
                            CountryShape(pathData: usa.path)
                                .stroke(Color.white, lineWidth: 0.5)
                        )
                        .frame(width: geo.size.width, height: geo.size.height)
                } else {
                    Text("🇺🇸 United States not found.")
                        .foregroundColor(.red)
                }
            }
            .background(Color.black.opacity(0.1))
        }
    }
}
