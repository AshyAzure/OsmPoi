using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices.WindowsRuntime;
using Windows.Foundation;
using Windows.Foundation.Collections;
using Windows.UI.Xaml;
using Windows.UI.Xaml.Controls;
using Windows.UI.Xaml.Controls.Primitives;
using Windows.UI.Xaml.Data;
using Windows.UI.Xaml.Input;
using Windows.UI.Xaml.Media;
using Windows.UI.Xaml.Navigation;
using System.Collections.ObjectModel;

// The Blank Page item template is documented at https://go.microsoft.com/fwlink/?LinkId=234238

namespace OsmPoi_for_Windows
{
    /// <summary>
    /// An empty page that can be used on its own or navigated to within a Frame.
    /// </summary>
    public sealed partial class DatasetPage : Page
    {
        public DatasetPage()
        {
            this.InitializeComponent();
            datasets.Add(new Dataset("D1"));
            datasets.Add(new Dataset("D2"));
        }

        private void AppBarButton_Click(object sender, RoutedEventArgs e)
        {

        }

        private ObservableCollection<Dataset> datasets = new ObservableCollection<Dataset>();

        public ObservableCollection<Dataset> Datasets { get { return datasets; } }
    }

    public class Dataset
    {
        public Dataset(string name)
        {
            Name = name;
        }

        public string Name { get; set; } 
    }
}
