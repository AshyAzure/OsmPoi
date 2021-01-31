//
//  ContentView.swift
//  OsmPoi for MacOS
//
//  Created by 王维境 on 2021/1/28.
//

import SwiftUI

enum ModeSelection {
    case query
    case datasets
}

struct ContentView: View {
    
    @State private var selection: ModeSelection? = .query
    
    var body: some View {
        NavigationView {
            List {
                // Query POI
                NavigationLink(
                    destination: QueryView(modelSelection: $selection).toolbar {
                        Spacer()
                    },
                    tag: ModeSelection.query,
                    selection: $selection)
                {
                    Label("查询POI", systemImage: "magnifyingglass")
                }
                // Datasets
                NavigationLink(
                    destination: DatasetsView(),
                    tag: ModeSelection.datasets,
                    selection: $selection)
                {
                    Label("管理数据", systemImage: "internaldrive")
                }
            }
            .listStyle(SidebarListStyle())
        }
        
    }

}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}


extension View {
    func `if`<Content: View>(_ conditional: Bool, content: (Self) -> Content) -> TupleView<(Self?, Content?)> {
        if conditional {
            return TupleView((nil, content(self)))
        } else {
            return TupleView((self, nil))
        }
    }
}
