import SwiftUI

struct WorldMapView: View {
    @Binding var selectedCountry: CountryData?
    private let countries: [CountryData] = loadCountries()

    init(selectedCountry: Binding<CountryData?>) {
        self._selectedCountry = selectedCountry
    }

    var body: some View {
        GeometryReader { geo in
            ZStack {
                ForEach(countries) { country in
                    let shape = CountryShape(pathData: country.path)

                    shape
                        .fill(color(for: country.advisory_level)) // Fill with advisory color
                        .overlay(
                            shape
                                .stroke(Color.black, lineWidth: 0.5) // Black border
                        )
                        .contentShape(shape) // Enables tapping
                        .onTapGesture {
                            selectedCountry = country
                        }
                }
            }
            .frame(width: geo.size.width, height: geo.size.height)
        }
    }

    // Color mapping for advisory levels
    private func color(for level: String) -> Color {
        switch level {
        case "Level 4": return .red
        case "Level 3": return .orange
        case "Level 2": return .yellow
        case "Level 1": return .green
        case "Level 0": return .white
        default: return .white
        }
    }
}

