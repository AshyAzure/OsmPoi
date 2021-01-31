//
//  QueryView.swift
//  OsmPoi for MacOS
//
//  Created by 王维境 on 2021/1/30.
//

import Foundation
import SwiftUI

struct QueryView: View {
    
    @State var selectedDataset: URL?
    @State var input: URL?
    @State var output: URL?
    
    @Binding var modelSelection: ModeSelection?
    
    @ObservedObject var dataDirectory = DataDirectory()
    
    var body: some View {
        if dataDirectory.files.count == 0 {
            Button("缺少数据集，前往添加数据", action: { modelSelection = .datasets })
                .toolbar(content: {
                    Spacer()
                })
        } else {
            VStack {
                Picker("数据集：", selection: $selectedDataset) {
                    ForEach(dataDirectory.files) { url in
                        Text(url.deletingPathExtension().deletingPathExtension().lastPathComponent).tag(url as URL?)
                    }
                }
                .padding()
                Button("查询输入", action: {
                    let panel = NSOpenPanel()
                    panel.canChooseDirectories = false
                    panel.allowsMultipleSelection = false
                    if panel.runModal() == .OK {
                        input = panel.url!
                    }
                })
                .padding()
                Button("查询输出", action: {
                    let panel = NSSavePanel()
                    if panel.runModal() == .OK {
                        output = panel.url!
                    }
                })
                Button("查询", action: {
                    guard input != nil, output != nil, selectedDataset != nil else { return }
                    osmpoi_query_csv(input!.path, output!.path, selectedDataset!.path, 1, false)
                    print("233333")
                    
                })
            }
        }
    }
}

struct QueryView_Previews: PreviewProvider {
    static var previews: some View {
        QueryView(modelSelection: .constant(.query))
    }
}
