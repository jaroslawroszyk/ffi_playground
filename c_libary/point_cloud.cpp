#include <vector>

struct Point
{
    double x;
    double y;
    double z;
};

class PointCloud
{
private:
    std::vector<Point> points;

public:
    void add_point(double x, double y, double z)
    {
        points.push_back({x, y, z});
    }

    std::vector<Point> get_points() const
    {
        return points;
    }

    Point centroid() const
    {
        Point c{0, 0, 0};
        if (points.empty())
            return c;
        for (auto &p : points)
        {
            c.x += p.x;
            c.y += p.y;
            c.z += p.z;
        }
        double n = points.size();
        c.x /= n;
        c.y /= n;
        c.z /= n;
        return c;
    }
};

extern "C"
{
    PointCloud *point_cloud_new() { return new PointCloud(); }
    void point_cloud_free(PointCloud *pc) { delete pc; }
    void point_cloud_add(PointCloud *pc, double x, double y, double z) { pc->add_point(x, y, z); }
    void point_cloud_centroid(PointCloud *pc, double *out)
    {
        auto c = pc->centroid();
        out[0] = c.x;
        out[1] = c.y;
        out[2] = c.z;
    }
}