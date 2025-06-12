#include <bits/stdc++.h>
using namespace std;
const double INF=1000000000000;
const int SIZE=500005;
#define ll long long

int n,m,S,cnt=0;
bool vis[SIZE];
int head[SIZE],pre[SIZE];
double dist[SIZE];

priority_queue<pair<double,int> > q;

struct edge{
	int next,to;
	double w;
}e[SIZE];

void add_edge(int x,int y,double z){
	e[++cnt].to=y;
	e[cnt].w=z;
	e[cnt].next=head[x];
	head[x]=cnt;
}

struct Node{
	int id;
	double x,y;
}node[SIZE];

double dis(double x1,double y1,double x2,double y2){
	return sqrt((x1-x2)*(x1-x2)+(y1-y2)*(y1-y2));
}

void print_path(int T) {
    vector<int> path;
    for (int v=T;v!=-1;v=pre[v]) {
        path.push_back(v);
    }
    reverse(path.begin(), path.end());
    for (int v:path) {
        cout<<v<<" ";
    }
    cout<<endl;
}

int main()
{
	freopen("usa.txt","r",stdin);
	scanf("%d%d",&n,&m);
	int S=1;
	for (int i=1;i<=n;i++){
		int z;
		double x,y;
		cin>>z>>x>>y;
		node[i-1].id=z;
		node[i-1].x=x; node[i-1].y=y;
	}
//	cout<<node[1].x<<' '<<node[1].y<<endl;
//	cout<<node[4].x<<' '<<node[4].y<<endl;
//	cout<<dis(node[1].x,node[1].y,node[4].x,node[4].y)<<endl;
	for (int i=1;i<=m;i++){
		int x,y;
		scanf("%d%d",&x,&y);
		double z=dis(node[x].x,node[x].y,node[y].x,node[y].y);
		//cout<<x<<' '<<y<<endl;
		add_edge(x,y,z);
		add_edge(y,x,z);
	}
	
	memset(vis,false,sizeof(vis));
	for (int i=0;i<SIZE;i++) {
	    dist[i]=INF;
	    pre[i]=-1;  
	}
	dist[S]=0;

	q.push(make_pair(0,S));
	
	while (q.size()){
		int x=q.top().second; q.pop();
		if (vis[x]) continue;
		vis[x]=true;
		for (int i=head[x];i;i=e[i].next){
			if (dist[e[i].to]>e[i].w+dist[x]){
				dist[e[i].to]=e[i].w+dist[x];
				pre[e[i].to] = x;  
				q.push(make_pair(-dist[e[i].to],e[i].to));
			}
		}
	}
//	
//	for (int i=1;i<=n;i++)
//		printf("%lf ",dist[i]);
	int T=8000;
	cout<<dist[T]<<endl;
	print_path(T);
	return 0;
}
