import SwiftUI

struct MapView: View {
    @State private var countries: [CountryData] = loadCountries()
    @State private var selectedCountry: CountryData?

    var body: some View {
        GeometryReader { geo in
            ZStack {
                Image("WorldMap")
                                .resizable()
                                .aspectRatio(contentMode: .fit)
                                .frame(width: geo.size.width, height: geo.size.height)
                ForEach(countries) { country in
                    CountryShape(pathData: country.path)
                        .fill(Color.red.opacity(0.5))
                        .overlay(
                            CountryShape(pathData: country.path)
                                .stroke(Color.white, lineWidth: 0.5)
                        )
                        .onTapGesture {
                            selectedCountry = country
                        }
                }

                if let selected = selectedCountry {
                    VStack {
                        Spacer()
                        VStack(spacing: 10) {
                            Text(selected.name)
                                .font(.title2)
                                .bold()
                                .padding(.top)

                            Text("Travel Advisory: \(selected.advisory_level)")
                                .font(.subheadline)
                                .foregroundColor(.red)

                            ScrollView {
                                Text(selected.advisory_text)
                                    .font(.body)
                                    .padding()
                            }
                        }
                        .frame(maxWidth: .infinity)
                        .background(Color.white.opacity(0.95))
                        .cornerRadius(16)
                        .padding()
                        .shadow(radius: 10)

                        Button("Close") {
                            selectedCountry = nil
                        }
                        .padding(.bottom)
                    }
                }
            }
            .background(Color.gray.opacity(0.1))
            .frame(width: geo.size.width, height: geo.size.height)
        }
    }
}

struct ContentView: View {
    var body: some View {
        MapView()
    }
}

#Preview {
    ContentView()
}
