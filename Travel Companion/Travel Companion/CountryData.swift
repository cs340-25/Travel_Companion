import SwiftUI

struct CountryData: Codable, Identifiable {
    var id: String { code }
    let code: String
    let name: String
    let path: String
    let advisory_level: String
    let advisory_text: String
}

func loadCountries() -> [CountryData] {
    guard let url = Bundle.main.url(forResource: "combined", withExtension: "json"),
          let data = try? Data(contentsOf: url),
          let countries = try? JSONDecoder().decode([CountryData].self, from: data) else {
        return []
    }
    print("Loaded countries: \(countries.count)")
    if let first = countries.first {
        print("First country code: \(first.code)")
        print("First country path: \(first.path.prefix(100))...")
    }
    return countries
}
