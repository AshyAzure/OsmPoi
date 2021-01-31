//
//  DatasetView.swift
//  OsmPoi for MacOS
//
//  Created by 王维境 on 2021/1/30.
//

import Foundation
import SwiftUI

struct DatasetsView: View {
    
    @ObservedObject var dataDirectory = DataDirectory()
    @State var selection: URL? = nil
    
    var body: some View {
        List {
            GeometryReader { geo in
                HStack {
                    Text("名称")
                        .frame(width: geo.size.width * 0.8, alignment: .leading)
                    Text("文件大小")
                }
            }
            ForEach(dataDirectory.files) { url in
                GeometryReader { geo in
                    HStack {
                        Text(url.relativeString)
                            .frame(width: geo.size.width * 0.8, alignment: .leading)
                        Text("\(url.fileSize)")
                    }
                }
                .onTapGesture {
                    selection = url
                }
                .contextMenu {
                    Button("删除", action: {
                        dataDirectory.deleteFile(url: url)
                    })
                }
                .listRowBackground(selection == url ? Color.accentColor : Color.clear)
                
            }
        }
//        .listStyle(PlainListStyle())
        .toolbar {
            ToolbarItem {
                Button(action: dataDirectory.createFile, label: {
                    Image(systemName: "plus")
                })
            }
            ToolbarItem {
                Button(action:  {
                    if let selection = selection {
                        dataDirectory.deleteFile(url: selection)
                    }
                }, label: {
                    Image(systemName: "trash")
                })
                .disabled(dataDirectory.files.count == 0)
            }
        }
    }
}
