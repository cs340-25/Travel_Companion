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
    guard let url = Bundle.main.url(forResource: "fixed_combined_countries", withExtension: "json"),
          let data = try? Data(contentsOf: url),
          let countries = try? JSONDecoder().decode([CountryData].self, from: data) else {
        return []
    }
    return countries
}
